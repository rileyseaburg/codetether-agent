#!/usr/bin/env python3
"""Reproduce the OpenRouter 400 by matching our real streaming request shape.

The non-streaming session-title call succeeds; the agent's streaming call fails
with HTTP 400. That call sends 3 messages (system + user + ...) plus the full
tool catalog and `stream: true`. This walks the differences one at a time to
isolate the rejected field, printing the full `error` object (including
`metadata`, which our parser currently drops).

Usage:
    OPENROUTER_API_KEY=... python3 openrouter_stream_probe.py [model]
"""

import json
import os
import sys
import urllib.error
import urllib.request

URL = "https://openrouter.ai/api/v1/chat/completions"
DEFAULT_MODEL = "google/gemma-4-26b-a4b-it:free"

SYSTEM = "You are CodeTether Agent, an expert software-engineering agent."


def api_key():
    """Return the OpenRouter key from the environment."""
    value = os.environ.get("OPENROUTER_API_KEY", "").strip()
    if not value:
        print("set OPENROUTER_API_KEY", file=sys.stderr)
        raise SystemExit(2)
    return value


def tool(name, extra_props=None):
    """Build one function tool definition."""
    props = {"path": {"type": "string", "description": "A path."}}
    if extra_props:
        props.update(extra_props)
    return {
        "type": "function",
        "function": {
            "name": name,
            "description": "A probe tool.",
            "parameters": {
                "type": "object",
                "properties": props,
                "required": ["path"],
            },
        },
    }


def send(model, body, label):
    """POST one body and report status plus the full error object."""
    payload = dict(body)
    payload["model"] = model
    req = urllib.request.Request(
        URL,
        data=json.dumps(payload).encode(),
        headers={
            "Authorization": f"Bearer {api_key()}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://codetether.run",
            "X-Title": "CodeTether Agent",
        },
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            raw = resp.read().decode("utf-8", "replace")
            first = raw.splitlines()[0] if raw.splitlines() else ""
            print(f"  {label:34} 200  {first[:70]}")
            return 200
    except urllib.error.HTTPError as exc:
        raw = exc.read().decode("utf-8", "replace")
        print(f"  {label:34} {exc.code}")
        try:
            parsed = json.loads(raw)
            err = parsed.get("error", parsed)
            print(f"      {json.dumps(err)[:600]}")
        except json.JSONDecodeError:
            print(f"      raw: {raw[:300]}")
        return exc.code
    except OSError as exc:
        print(f"  {label:34} transport {type(exc).__name__}")
        return 0


def main():
    model = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_MODEL
    print(f"model: {model}")

    base_msgs = [
        {"role": "system", "content": SYSTEM},
        {"role": "user", "content": "hello"},
    ]
    one_tool = [tool("list")]
    many_tools = [tool(f"probe_{i}") for i in range(40)]

    print("\n-- stream flag --")
    send(model, {"messages": base_msgs, "stream": False}, "no stream, no tools")
    send(model, {"messages": base_msgs, "stream": True}, "stream, no tools")

    print("\n-- tools with stream --")
    send(model, {"messages": base_msgs, "stream": True, "tools": one_tool}, "stream + 1 tool")
    send(model, {"messages": base_msgs, "stream": True, "tools": many_tools}, "stream + 40 tools")

    print("\n-- schema features --")
    nested = [
        tool(
            "nested",
            {
                "items": {"type": "array", "items": {"type": "string"}},
                "mode": {"type": "string", "enum": ["a", "b"]},
            },
        )
    ]
    send(model, {"messages": base_msgs, "stream": True, "tools": nested}, "stream + nested schema")

    no_required = [
        {
            "type": "function",
            "function": {
                "name": "no_required",
                "description": "No required array.",
                "parameters": {"type": "object", "properties": {}},
            },
        }
    ]
    send(
        model,
        {"messages": base_msgs, "stream": True, "tools": no_required},
        "stream + empty properties",
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
