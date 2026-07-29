#!/usr/bin/env python3
"""Realistic-load Codex backend probe.

The trivial "say ok" probe showed identity headers make no difference. This probe
tests the conditions that actually differ between our client and the Codex CLI on
a real turn: large tool schemas, reasoning config, `include`
(`reasoning.encrypted_content`), `store`, and long-running streams that must
survive to `response.completed`.

Records time-to-first-byte, total duration, event counts, and terminal reason so
mid-stream truncation is visible rather than inferred.

Usage:
    PROBE_MODEL=gpt-5.6-luna python3 codex_load_probe.py [variant ...]
"""

import json
import os
import pathlib
import sys
import time
import urllib.error
import urllib.request


AUTH = pathlib.Path.home() / ".codex" / "auth.json"
URL = "https://chatgpt.com/backend-api/codex/responses"
MODEL = os.environ.get("PROBE_MODEL", "gpt-5.6-luna")


def load_token():
    data = json.loads(AUTH.read_text())
    return data["tokens"]["access_token"], data["tokens"]["account_id"]


def tool_schema(count):
    tools = []
    for index in range(count):
        tools.append(
            {
                "type": "function",
                "name": f"probe_tool_{index}",
                "description": "Probe tool used to size the request envelope.",
                "strict": False,
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "A file path."},
                        "content": {"type": "string", "description": "Body text."},
                    },
                    "required": ["path"],
                },
            }
        )
    return tools


def build_body(variant):
    prompt = variant.get("prompt", "say ok")
    body = {
        "model": MODEL,
        "instructions": variant.get("instructions", "Reply concisely."),
        "input": [
            {
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": prompt}],
            }
        ],
        "stream": True,
        "store": variant.get("store", False),
    }
    if "include" in variant:
        body["include"] = variant["include"]
    if variant.get("tools"):
        body["tools"] = tool_schema(variant["tools"])
        body["tool_choice"] = "auto"
        body["parallel_tool_calls"] = True
    if variant.get("reasoning"):
        body["reasoning"] = variant["reasoning"]
    if variant.get("prompt_cache_key"):
        body["prompt_cache_key"] = variant["prompt_cache_key"]
    return body


def probe(name, variant):
    token, account = load_token()
    headers = {
        "Authorization": f"Bearer {token}",
        "chatgpt-account-id": account,
        "Content-Type": "application/json",
        "Accept": "text/event-stream",
    }
    headers.update(variant.get("headers", {}))
    payload = json.dumps(build_body(variant)).encode()
    req = urllib.request.Request(URL, data=payload, headers=headers, method="POST")
    result = {
        "variant": name,
        "model": MODEL,
        "request_bytes": len(payload),
        "tools": variant.get("tools", 0),
    }
    started = time.monotonic()
    try:
        with urllib.request.urlopen(req, timeout=300) as resp:
            result["status"] = resp.status
            result["ttfb_s"] = round(time.monotonic() - started, 2)
            counts = {}
            terminal = None
            last_event_at = time.monotonic()
            max_gap = 0.0
            for raw in resp:
                line = raw.decode("utf-8", "replace").strip()
                if not line.startswith("data:"):
                    continue
                now = time.monotonic()
                max_gap = max(max_gap, now - last_event_at)
                last_event_at = now
                chunk = line[5:].strip()
                if chunk == "[DONE]":
                    terminal = terminal or "done_sentinel"
                    break
                try:
                    event = json.loads(chunk)
                except json.JSONDecodeError:
                    continue
                etype = event.get("type", "?")
                counts[etype] = counts.get(etype, 0) + 1
                if etype == "response.completed":
                    terminal = "completed"
                    result["usage"] = event.get("response", {}).get("usage") or {}
                elif etype in ("response.failed", "response.incomplete", "error"):
                    terminal = etype
                    result["terminal_payload"] = str(event)[:300]
            result["duration_s"] = round(time.monotonic() - started, 2)
            result["max_event_gap_s"] = round(max_gap, 2)
            result["event_counts"] = counts
            result["terminal"] = terminal or "EOF_WITHOUT_TERMINAL"
    except urllib.error.HTTPError as exc:
        result["status"] = exc.code
        result["error_body"] = exc.read().decode("utf-8", "replace")[:400]
    except Exception as exc:
        result["status"] = "transport_error"
        result["error_body"] = f"{type(exc).__name__}: {exc}"
        result["duration_s"] = round(time.monotonic() - started, 2)
    return result


LONG_TASK = (
    "Think step by step and then write a detailed technical explanation, at least "
    "700 words, of how HTTP/2 stream multiplexing interacts with TLS record "
    "boundaries and TCP congestion control. Include concrete failure modes."
)

VARIANTS = {
    # Minimal control.
    "control_small": {"prompt": "say ok"},
    # Our exact production envelope.
    "codetether_envelope": {
        "prompt": "say ok",
        "include": ["reasoning.encrypted_content"],
        "reasoning": {"effort": "medium"},
        "prompt_cache_key": "probe-cache-key",
        "tools": 12,
    },
    # Long generation: does the stream survive to completion?
    "long_stream": {
        "prompt": LONG_TASK,
        "include": ["reasoning.encrypted_content"],
        "reasoning": {"effort": "medium"},
    },
    # Long generation with high reasoning effort and a big tool set.
    "long_stream_heavy": {
        "prompt": LONG_TASK,
        "include": ["reasoning.encrypted_content"],
        "reasoning": {"effort": "high"},
        "tools": 40,
        "prompt_cache_key": "probe-cache-key",
    },
    # Same heavy load, but without `include` to isolate encrypted reasoning.
    "long_stream_no_include": {
        "prompt": LONG_TASK,
        "reasoning": {"effort": "high"},
        "tools": 40,
    },
}


def main():
    only = sys.argv[1:] or list(VARIANTS)
    out = []
    for name in only:
        if name not in VARIANTS:
            print(f"skip unknown variant: {name}", file=sys.stderr)
            continue
        res = probe(name, VARIANTS[name])
        out.append(res)
        print(
            f"{name:24s} status={res.get('status')} terminal={res.get('terminal')} "
            f"ttfb={res.get('ttfb_s')}s dur={res.get('duration_s')}s "
            f"max_gap={res.get('max_event_gap_s')}s bytes={res['request_bytes']}",
            flush=True,
        )
        if res.get("error_body"):
            print(f"    error: {res['error_body'][:200]}", flush=True)
        if res.get("event_counts"):
            top = sorted(res["event_counts"].items(), key=lambda kv: -kv[1])[:5]
            print(f"    events: {top}", flush=True)
    dest = pathlib.Path(os.environ.get("PROBE_OUT", "/tmp/codex_load_probe.json"))
    dest.write_text(json.dumps(out, indent=2))
    print(f"\nwrote {dest}")


if __name__ == "__main__":
    main()
