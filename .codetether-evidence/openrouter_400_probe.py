#!/usr/bin/env python3
"""Find why OpenRouter rejects a request with HTTP 400.

Users report `google/gemma-4-26b-a4b-it:free` and
`nvidia/nemotron-3.5-content-safety:free` failing with:

    OpenRouter API error: Provider returned error (code: 400)

and, for one model:

    No endpoints found that support tool use (code: 404)

Our error parser reads only `error.message` and drops `error.metadata`, which is
where OpenRouter puts the upstream provider's real reason. This sends the same
request shapes our provider builds and prints the *full* error payload, so the
rejected field is identified rather than guessed.

Reads the key from the environment or Vault; never prints it.

Usage:
    OPENROUTER_API_KEY=... python3 openrouter_400_probe.py [model]
"""

import json
import os
import sys
import urllib.error
import urllib.request

URL = "https://openrouter.ai/api/v1/chat/completions"
DEFAULT_MODEL = "google/gemma-4-26b-a4b-it:free"

TOOLS = [
    {
        "type": "function",
        "function": {
            "name": "list",
            "description": "List a directory.",
            "parameters": {
                "type": "object",
                "properties": {"path": {"type": "string"}},
                "required": ["path"],
            },
        },
    }
]


def api_key():
    """Return the OpenRouter key from the environment."""
    for name in ("OPENROUTER_API_KEY", "OPENROUTER_KEY"):
        value = os.environ.get(name)
        if value and value.strip():
            return value.strip()
    print("set OPENROUTER_API_KEY", file=sys.stderr)
    raise SystemExit(2)


def post(model: str, body_extra: dict, label: str):
    """Send one variant and print the full error payload."""
    body = {
        "model": model,
        "messages": [{"role": "user", "content": "hello"}],
        "stream": False,
    }
    body.update(body_extra)
    req = urllib.request.Request(
        URL,
        data=json.dumps(body).encode(),
        headers={
            "Authorization": f"Bearer {api_key()}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://codetether.run",
            "X-Title": "CodeTether Agent",
        },
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=90) as resp:
            payload = json.loads(resp.read())
            choice = (payload.get("choices") or [{}])[0]
            text = (choice.get("message") or {}).get("content") or ""
            print(f"{label:26} 200  reply={text[:60]!r}")
    except urllib.error.HTTPError as exc:
        raw = exc.read().decode("utf-8", "replace")
        print(f"{label:26} {exc.code}")
        try:
            parsed = json.loads(raw)
            print(f"    {json.dumps(parsed.get('error', parsed), indent=6)[:900]}")
        except json.JSONDecodeError:
            print(f"    raw: {raw[:400]}")
    except OSError as exc:
        print(f"{label:26} transport_error {type(exc).__name__}: {exc}")


def main():
    model = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_MODEL
    print(f"model: {model}\n")
    post(model, {}, "no tools")
    post(model, {"tools": TOOLS}, "with tools")
    post(model, {"tools": TOOLS, "tool_choice": "auto"}, "tools + tool_choice")
    # Our provider sends an empty-content assistant turn when a prior reply had
    # only Thinking parts; check whether that alone is rejected.
    post(
        model,
        {
            "messages": [
                {"role": "user", "content": "hello"},
                {"role": "assistant", "content": ""},
                {"role": "user", "content": "hello"},
            ]
        },
        "empty assistant turn",
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
