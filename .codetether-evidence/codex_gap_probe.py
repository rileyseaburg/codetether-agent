#!/usr/bin/env python3
"""Measure real Codex reasoning silence gaps.

Hypothesis under test: CodeTether's shared HTTP client sets
``TCP_USER_TIMEOUT = 60s`` and TCP keepalive (30s idle + 3x10s probes). During a
long reasoning phase the backend sends no SSE bytes, so a genuinely healthy
stream can look like a dead peer and be killed, producing the "terminates
mid-stream" symptom.

This probe forces heavy reasoning and records the largest inter-event gap plus
the pre-first-token silence, so measured silence can be compared against the
60-second timeout instead of guessed at.

Usage:
    PROBE_MODEL=gpt-5.6-luna python3 codex_gap_probe.py
"""

import json
import os
import pathlib
import time
import urllib.error
import urllib.request


AUTH = pathlib.Path.home() / ".codex" / "auth.json"
URL = "https://chatgpt.com/backend-api/codex/responses"
MODEL = os.environ.get("PROBE_MODEL", "gpt-5.6-luna")
VISIBLE_EVENTS = (
    "response.output_text.delta",
    "response.reasoning_summary_text.delta",
)
TERMINAL_FAILURES = ("response.failed", "response.incomplete", "error")

HARD_TASK = (
    "Solve this precisely and show rigorous reasoning before answering. "
    "Consider a 19x19 grid graph. Derive an exact argument for the number of "
    "monotone lattice paths from corner to corner that avoid the two main "
    "diagonals entirely, verify your count with an independent combinatorial "
    "identity, then compute the final integer exactly. Do not answer until you "
    "have double-checked with a second method."
)


def load_token():
    """Return the ChatGPT access token and account id from Codex auth."""
    data = json.loads(AUTH.read_text())
    return data["tokens"]["access_token"], data["tokens"]["account_id"]


def build_request(effort: str, summary: bool):
    """Build the Responses request for the given reasoning configuration."""
    token, account = load_token()
    reasoning = {"effort": effort}
    if summary:
        reasoning["summary"] = "auto"
    body = {
        "model": MODEL,
        "instructions": "You are a careful mathematician.",
        "input": [
            {
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": HARD_TASK}],
            }
        ],
        "stream": True,
        "store": False,
        "include": ["reasoning.encrypted_content"],
        "reasoning": reasoning,
    }
    return urllib.request.Request(
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


def run(effort: str, summary: bool):
    """Stream one turn and report silence characteristics."""
    label = f"effort={effort} summary={summary}"
    started = time.monotonic()
    first_data_at = None
    first_visible_at = None
    max_gap = 0.0
    gap_after = None
    prev_type = None
    counts = {}
    terminal = "EOF_WITHOUT_TERMINAL"
    try:
        request = build_request(effort, summary)
        with urllib.request.urlopen(request, timeout=600) as resp:
            last_at = time.monotonic()
            for raw in resp:
                line = raw.decode("utf-8", "replace").strip()
                if not line.startswith("data:"):
                    continue
                now = time.monotonic()
                if first_data_at is None:
                    first_data_at = now
                if now - last_at > max_gap:
                    max_gap = now - last_at
                    gap_after = prev_type
                last_at = now
                chunk = line[5:].strip()
                if chunk == "[DONE]":
                    break
                try:
                    event = json.loads(chunk)
                except json.JSONDecodeError:
                    continue
                etype = event.get("type", "?")
                prev_type = etype
                counts[etype] = counts.get(etype, 0) + 1
                if etype in VISIBLE_EVENTS and first_visible_at is None:
                    first_visible_at = now
                if etype == "response.completed":
                    terminal = "completed"
                elif etype in TERMINAL_FAILURES:
                    terminal = etype
    except urllib.error.HTTPError as exc:
        return {
            "label": label,
            "status": exc.code,
            "error": exc.read().decode("utf-8", "replace")[:300],
        }
    except OSError as exc:
        return {
            "label": label,
            "status": "transport_error",
            "error": f"{type(exc).__name__}: {exc}",
            "elapsed_s": round(time.monotonic() - started, 2),
            "max_gap_s": round(max_gap, 2),
        }
    return {
        "label": label,
        "status": 200,
        "terminal": terminal,
        "total_s": round(time.monotonic() - started, 2),
        "ttfb_s": round((first_data_at or started) - started, 2),
        "time_to_first_visible_s": (
            round(first_visible_at - started, 2) if first_visible_at else None
        ),
        "max_event_gap_s": round(max_gap, 2),
        "max_gap_after_event": gap_after,
        "event_counts": dict(sorted(counts.items(), key=lambda kv: -kv[1])[:6]),
    }


def main():
    """Run the with/without-summary comparison and persist results."""
    results = []
    for effort, summary in (("high", False), ("high", True)):
        res = run(effort, summary)
        results.append(res)
        print(json.dumps(res, indent=2), flush=True)
    default_out = "/tmp/codex_gap_probe.json"
    dest = pathlib.Path(os.environ.get("PROBE_OUT", default_out))
    dest.write_text(json.dumps(results, indent=2))
    print(f"\nwrote {dest}")


if __name__ == "__main__":
    main()
