#!/usr/bin/env python3
"""A/B the intermittent bare `error` event with and without `reasoning.summary`.

The single-shot probe showed `gpt-5.6-sol` intermittently emits a bare
`{"type":"error"}` event with no populated fields. This runs repeated trials in
both configurations and dumps the raw event so the failure can be characterized
rather than guessed at.

Usage:
    python3 codex_error_ab.py gpt-5.6-sol 6
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
    try:
        with urllib.request.urlopen(req, timeout=180) as resp:
            request_id = resp.headers.get("x-request-id")
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
                if etype == "response.completed":
                    return {
                        "outcome": "completed",
                        "elapsed_s": round(time.monotonic() - started, 2),
                        "request_id": request_id,
                    }
                if etype in ("error", "response.failed", "response.incomplete"):
                    return {
                        "outcome": etype,
                        "raw_event": json.dumps(event)[:600],
                        "elapsed_s": round(time.monotonic() - started, 2),
                        "request_id": request_id,
                    }
            return {
                "outcome": "EOF_WITHOUT_TERMINAL",
                "elapsed_s": round(time.monotonic() - started, 2),
                "request_id": request_id,
            }
    except urllib.error.HTTPError as exc:
        return {
            "outcome": f"http_{exc.code}",
            "raw_event": exc.read().decode("utf-8", "replace")[:400],
            "elapsed_s": round(time.monotonic() - started, 2),
        }
    except Exception as exc:
        return {
            "outcome": "transport_error",
            "raw_event": f"{type(exc).__name__}: {exc}",
            "elapsed_s": round(time.monotonic() - started, 2),
        }


def main():
    model = sys.argv[1] if len(sys.argv) > 1 else "gpt-5.6-sol"
    trials = int(sys.argv[2]) if len(sys.argv) > 2 else 6
    tally = {}
    for summary in ("auto", None):
        key = f"summary={summary}"
        tally[key] = {}
        for index in range(trials):
            res = attempt(model, summary)
            outcome = res["outcome"]
            tally[key][outcome] = tally[key].get(outcome, 0) + 1
            print(
                f"{key:14s} run={index + 1} outcome={outcome} "
                f"elapsed={res['elapsed_s']}s rid={res.get('request_id')}",
                flush=True,
            )
            if res.get("raw_event"):
                print(f"    raw: {res['raw_event'][:400]}", flush=True)
    print("\ntally:", json.dumps(tally, indent=2))


if __name__ == "__main__":
    main()
