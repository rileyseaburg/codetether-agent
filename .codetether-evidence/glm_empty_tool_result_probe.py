#!/usr/bin/env python3
"""Find the GLM-5.2 edge case: does an empty tool result cause an empty stream?

Observed live in CodeTether: a `glob` call returned `(empty output)`, and the
next turn failed with:

    stream retry limit exhausted after 5 retries:
    Z.AI stream ended without producing any content (empty response)

`src/provider/zai.rs:265` forwards tool output verbatim:

    json!({"role": "tool", "tool_call_id": ..., "content": content})

so an empty tool result becomes `"content": ""`. This tests whether that is what
silences the model, by varying ONLY the tool content across otherwise identical
requests.

Credentials: API key from Vault. Never hardcoded, never printed.
"""

from __future__ import annotations

import json
import subprocess

import requests

BASE_URL = "https://api.z.ai/api/paas/v4/chat/completions"
VAULT_PATH = "secret/codetether/providers/glm5"
MODEL = "glm-5.2"

TOOL = {
    "type": "function",
    "function": {
        "name": "glob",
        "description": "Find files matching a glob pattern.",
        "parameters": {
            "type": "object",
            "properties": {"pattern": {"type": "string"}},
            "required": ["pattern"],
        },
    },
}

# Only the tool-result content differs between cases.
CASES = [
    ("empty string", ""),
    ("literal (empty output)", "(empty output)"),
    ("single space", " "),
    ("explicit no-match sentence", "No files matched the pattern."),
    ("normal result", "spotless-web/ui/components/input.tsx"),
]


def load_api_key() -> str:
    for field in ("api_key", "token"):
        probe = subprocess.run(
            ["vault", "kv", "get", f"-field={field}", VAULT_PATH],
            capture_output=True, text=True,
        )
        if probe.returncode == 0 and probe.stdout.strip():
            return probe.stdout.strip()
    raise RuntimeError(f"no api_key/token at {VAULT_PATH}")


def build_messages(tool_content: str) -> list[dict]:
    """A realistic two-turn history ending in a tool result."""
    return [
        {"role": "system", "content": "You are a senior software engineer."},
        {"role": "user", "content": "Find the catalyst input/select/button components."},
        {
            "role": "assistant",
            "content": "",
            "tool_calls": [
                {
                    "id": "call_1",
                    "type": "function",
                    "function": {
                        "name": "glob",
                        "arguments": json.dumps(
                            {"pattern": "spotless-web/ui/**/{input,select,button}.tsx"}
                        ),
                    },
                }
            ],
        },
        {"role": "tool", "tool_call_id": "call_1", "content": tool_content},
    ]


def stream_once(session: requests.Session, tool_content: str) -> dict:
    """Stream one completion; count what the provider actually emitted."""
    payload = {
        "model": MODEL,
        "messages": build_messages(tool_content),
        "tools": [TOOL],
        "stream": True,
        "temperature": 1.0,
    }
    reasoning_chars = 0
    content_chars = 0
    tool_calls = 0
    saw_done = False

    with session.post(BASE_URL, json=payload, stream=True, timeout=180) as response:
        if response.status_code != 200:
            return {"http": response.status_code, "error": response.text[:200]}
        for raw in response.iter_lines(decode_unicode=True):
            if not raw or not raw.startswith("data: "):
                continue
            data = raw[6:].strip()
            if data == "[DONE]":
                saw_done = True
                break
            try:
                delta = json.loads(data)["choices"][0].get("delta", {})
            except (json.JSONDecodeError, KeyError, IndexError):
                continue
            reasoning_chars += len(delta.get("reasoning_content") or "")
            content_chars += len(delta.get("content") or "")
            tool_calls += len(delta.get("tool_calls") or [])

    produced = bool(reasoning_chars or content_chars or tool_calls)
    return {
        "http": 200,
        "reasoning_chars": reasoning_chars,
        "content_chars": content_chars,
        "tool_calls": tool_calls,
        "saw_done": saw_done,
        "produced_output": produced,
    }


def main() -> int:
    session = requests.Session()
    session.headers.update({
        "Authorization": f"Bearer {load_api_key()}",
        "Content-Type": "application/json",
    })

    print(f"{'tool result content':30s}{'reason':>8s}{'text':>7s}{'calls':>7s}  verdict")
    print("-" * 74)

    rows = []
    for label, content in CASES:
        result = stream_once(session, content)
        rows.append((label, result))
        if result.get("http") != 200:
            print(f"{label:30s}{'':22s}HTTP {result['http']}: {result.get('error','')[:30]}")
            continue
        verdict = "ok" if result["produced_output"] else "EMPTY STREAM <-- reproduced"
        print(f"{label:30s}{result['reasoning_chars']:>8d}"
              f"{result['content_chars']:>7d}{result['tool_calls']:>7d}  {verdict}")

    print("\n=== verdict ===")
    empties = [label for label, r in rows if r.get("http") == 200 and not r["produced_output"]]
    if empties:
        print("Empty stream reproduced for:", ", ".join(empties))
        print("Fix: substitute a non-empty placeholder for empty tool output")
        print("before sending, in provider/zai.rs convert_messages.")
    else:
        print("No empty stream reproduced; trigger is elsewhere.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())