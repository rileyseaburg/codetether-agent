#!/usr/bin/env python3
"""Verify `reasoning.summary` support across the models we actually offer.

The gap probe showed `reasoning.summary` reduces pre-token silence from ~48s to
~4s. Before sending it unconditionally we must confirm every supported model
accepts it, and confirm it really produces reasoning-summary events (which is
what keeps the stream warm).

Usage:
    python3 codex_summary_probe.py
"""

import json
import os
import pathlib
import time
import urllib.error
import urllib.request


AUTH = pathlib.Path.home() / ".codex" / "auth.json"
URL = "https://chatgpt.com/backend-api/codex/responses"

MODELS = ["gpt-5.6-luna", "gpt-5.6-sol", "gpt-5.6-terra", "gpt-5.5"]

TASK = (
    "Carefully determine the number of trailing zeros in 2025! and explain the "
    "reasoning using Legendre's formula. Verify the arithmetic twice."
)


def load_token():
    data = json.loads(AUTH.read_text())
    return data["tokens"]["access_token"], data["tokens"]["account_id"]


def run(model: str, summary: str | None, effort: str = "high"):
    token, account = load_token()
    reasoning = {"effort": effort}
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
    first_visible = None
    summary_events = 0
    counts = {}
    terminal = "EOF_WITHOUT_TERMINAL"
    try:
        with urllib.request.urlopen(req, timeout=420) as resp:
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
                etype = event.get("type", "?")
                counts[etype] = counts.get(etype, 0) + 1
                if "reasoning_summary" in etype:
                    summary_events += 1
                if (
                    etype
                    in (
                        "response.output_text.delta",
                        "response.reasoning_summary_text.delta",
                    )
                    and first_visible is None
                ):
                    first_visible = round(time.monotonic() - started, 2)
                if etype == "response.completed":
                    terminal = "completed"
                elif etype in ("response.failed", "response.incomplete", "error"):
                    terminal = etype
    except urllib.error.HTTPError as exc:
        return {
            "model": model,
            "summary": summary,
            "status": exc.code,
            "error": exc.read().decode("utf-8", "replace")[:250],
        }
    except Exception as exc:
        return {
            "model": model,
            "summary": summary,
            "status": "transport_error",
            "error": f"{type(exc).__name__}: {exc}",
        }
    return {
        "model": model,
        "summary": summary,
        "status": 200,
        "terminal": terminal,
        "first_visible_s": first_visible,
        "total_s": round(time.monotonic() - started, 2),
        "summary_events": summary_events,
    }


def main():
    results = []
    for model in MODELS:
        for summary in ("auto", None):
            res = run(model, summary)
            results.append(res)
            print(
                f"{model:16s} summary={summary!s:5s} status={res.get('status')} "
                f"terminal={res.get('terminal')} first_visible={res.get('first_visible_s')}s "
                f"summary_events={res.get('summary_events')} total={res.get('total_s')}s",
                flush=True,
            )
            if res.get("error"):
                print(f"    error: {res['error'][:200]}", flush=True)
    dest = pathlib.Path(os.environ.get("PROBE_OUT", "/tmp/codex_summary_probe.json"))
    dest.write_text(json.dumps(results, indent=2))
    print(f"\nwrote {dest}")


if __name__ == "__main__":
    main()
