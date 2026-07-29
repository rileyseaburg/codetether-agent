#!/usr/bin/env python3
"""Capture the exact terminal event for a model/summary combination.

Used to confirm whether `gpt-5.6-sol` reproducibly fails when `reasoning.summary`
is omitted, and to record the server's own failure payload as evidence.

Usage:
    python3 codex_failure_detail.py gpt-5.6-sol none 3
    python3 codex_failure_detail.py gpt-5.6-sol auto 1
"""

import json
import pathlib
import sys
import time
import urllib.error
import urllib.request


AUTH = pathlib.Path.home() / ".codex" / "auth.json"
URL = "https://chatgpt.com/backend-api/codex/responses"
TASK = (
    "Carefully determine the number of trailing zeros in 2025! and explain the "
    "reasoning using Legendre's formula. Verify the arithmetic twice."
)


def load_token():
    data = json.loads(AUTH.read_text())
    return data["tokens"]["access_token"], data["tokens"]["account_id"]


def attempt(model: str, summary: str | None):
    token, account = load_token()
    reasoning = {"effort": "high"}
    if summary is not None:
        reasoning["summary"] = summary
    body = {
        "model": model,
        "instructions": "Be rigorous.",
        "input": [
            {
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": TASK}],
            }
        ],
        "stream": True,
        "store": False,
        "include": ["reasoning.encrypted_content"],
        "reasoning": reasoning,
    }
    req = urllib.request.Request(
        URL,
        data=json.dumps(body).encode(),
        headers={
            "Authorization": f"Bearer {token}",
            "chatgpt-account-id": account,
            "Content-Type": "application/json",
            "Accept": "text/event-stream",
        },
        method="POST",
    )
    started = time.monotonic()
    terminal = None
    payload = None
    try:
        with urllib.request.urlopen(req, timeout=180) as resp:
            for raw in resp:
                line = raw.decode("utf-8", "replace").strip()
                if not line.startswith("data:"):
                    continue
                chunk = line[5:].strip()
                if chunk == "[DONE]":
                    break
                try:
                    event = json.loads(chunk)
                except json.JSONDecodeError:
                    continue
                etype = event.get("type", "")
                if etype in (
                    "response.failed",
                    "response.incomplete",
                    "error",
                    "response.completed",
                ):
                    terminal = etype
                    payload = event
                    if etype != "response.completed":
                        break
    except urllib.error.HTTPError as exc:
        return {
            "terminal": f"http_{exc.code}",
            "detail": exc.read().decode("utf-8", "replace")[:400],
            "elapsed_s": round(time.monotonic() - started, 2),
        }
    detail = None
    if payload and terminal != "response.completed":
        response = payload.get("response", {})
        detail = json.dumps(
            {
                "status": response.get("status"),
                "error": response.get("error"),
                "incomplete_details": response.get("incomplete_details"),
            }
        )[:500]
    return {
        "terminal": terminal or "EOF_WITHOUT_TERMINAL",
        "detail": detail,
        "elapsed_s": round(time.monotonic() - started, 2),
    }


def main():
    model = sys.argv[1] if len(sys.argv) > 1 else "gpt-5.6-sol"
    raw_summary = sys.argv[2].lower() if len(sys.argv) > 2 else "none"
    summary = None if raw_summary in ("none", "omit") else raw_summary
    runs = int(sys.argv[3]) if len(sys.argv) > 3 else 3
    for index in range(runs):
        result = attempt(model, summary)
        print(
            f"run={index + 1} model={model} summary={summary} "
            f"terminal={result['terminal']} elapsed={result['elapsed_s']}s",
            flush=True,
        )
        if result.get("detail"):
            print(f"    detail: {result['detail']}", flush=True)


if __name__ == "__main__":
    main()
