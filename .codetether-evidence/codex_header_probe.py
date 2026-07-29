#!/usr/bin/env python3
"""Empirical Codex-backend header probe.

Determines which request headers measurably change backend behavior for *our own*
client identity.

We deliberately do NOT send Codex's first-party ``originator``/``User-Agent``
values (``codex_cli_rs``, ``codex-tui``, ...). Upstream
``is_first_party_originator`` treats those as identity claims, so copying them
would be impersonation rather than protocol parity.

Result (recorded in probe_headers.json): all variants, including a bare request
with no identity headers at all, receive identical ``200`` responses and
identical ``x-codex-active-limit``/``x-codex-plan-type`` entitlements. There is
no header-based preferential treatment to mimic.

Usage:
    PROBE_MODEL=gpt-5.6-luna python3 codex_header_probe.py [variant ...]
"""

import json
import os
import pathlib
import sys
import urllib.error
import urllib.request


AUTH = pathlib.Path.home() / ".codex" / "auth.json"
URL = "https://chatgpt.com/backend-api/codex/responses"
DEFAULT_MODEL = os.environ.get("PROBE_MODEL", "gpt-5.6-luna")
INTERESTING = ("ratelimit", "x-codex", "x-request-id", "processing")


def load_token():
    """Return the ChatGPT access token and account id from Codex auth."""
    data = json.loads(AUTH.read_text())
    tokens = data["tokens"]
    return tokens["access_token"], tokens["account_id"]


def body(model):
    """Build a minimal streaming Responses payload."""
    return {
        "model": model,
        "instructions": "Reply with the single word: ok",
        "input": [
            {
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": "say ok"}],
            }
        ],
        "stream": True,
        "store": False,
        "include": [],
    }


def _filter_headers(headers):
    """Keep only entitlement and diagnostic response headers."""
    return {
        key.lower(): value
        for key, value in headers.items()
        if any(tag in key.lower() for tag in INTERESTING)
    }


def probe(name, extra, model=DEFAULT_MODEL):
    """Send one variant and report status, entitlements, and completion."""
    token, account = load_token()
    headers = {
        "Authorization": f"Bearer {token}",
        "chatgpt-account-id": account,
        "Content-Type": "application/json",
        "Accept": "text/event-stream",
    }
    headers.update(extra)
    req = urllib.request.Request(
        URL,
        data=json.dumps(body(model)).encode(),
        headers=headers,
        method="POST",
    )
    result = {"variant": name, "model": model, "sent": sorted(extra.keys())}
    try:
        with urllib.request.urlopen(req, timeout=90) as resp:
            result["status"] = resp.status
            result["resp_headers"] = _filter_headers(resp.headers)
            saw_completed = False
            deltas = 0
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
                if etype == "response.output_text.delta":
                    deltas += 1
                if etype == "response.completed":
                    saw_completed = True
                    result["usage"] = event.get("response", {}).get("usage") or {}
                if etype in ("response.failed", "error"):
                    result["stream_error"] = str(event)[:300]
            result["completed"] = saw_completed
            result["text_deltas"] = deltas
    except urllib.error.HTTPError as exc:
        result["status"] = exc.code
        result["error_body"] = exc.read().decode("utf-8", "replace")[:400]
        result["resp_headers"] = _filter_headers(exc.headers)
    except OSError as exc:
        result["status"] = "transport_error"
        result["error_body"] = f"{type(exc).__name__}: {exc}"
    return result


IDENTITY = {
    "originator": "codetether_agent_rs",
    "User-Agent": "codetether_agent_rs/4.7.5 (Linux; x86_64)",
}
THREAD = "11111111-1111-4111-8111-111111111111"

VARIANTS = {
    # What CodeTether sends today.
    "baseline_current": {"version": "0.144.0"},
    # No client-identity headers at all.
    "bare": {},
    # Our own honest identity.
    "codetether_identity": dict(IDENTITY),
    # Honest identity plus conversation grouping (structure, not identity).
    "identity_plus_session": {
        **IDENTITY,
        "session-id": THREAD,
        "thread-id": THREAD,
        "x-client-request-id": THREAD,
        "x-codex-window-id": f"{THREAD}:0",
    },
    # Does the backend honor timing metrics for us?
    "timing_metrics": {
        **IDENTITY,
        "x-responsesapi-include-timing-metrics": "true",
    },
    # Is an unknown beta feature key rejected or ignored?
    "beta_features": {
        **IDENTITY,
        "x-codex-beta-features": "responses_websockets",
    },
}


def main():
    """Run the requested variants and persist the comparison."""
    only = sys.argv[1:] or list(VARIANTS)
    out = []
    for name in only:
        if name not in VARIANTS:
            print(f"skip unknown variant: {name}", file=sys.stderr)
            continue
        res = probe(name, VARIANTS[name])
        out.append(res)
        resp_headers = res.get("resp_headers", {})
        print(
            f"{res['variant']:24s} status={res.get('status')} "
            f"completed={res.get('completed')} "
            f"deltas={res.get('text_deltas')} "
            f"limit={resp_headers.get('x-codex-active-limit')} "
            f"plan={resp_headers.get('x-codex-plan-type')}",
            flush=True,
        )
        if res.get("error_body"):
            print(f"    error: {res['error_body'][:200]}", flush=True)
    default_out = "/tmp/codex_header_probe.json"
    dest = pathlib.Path(os.environ.get("PROBE_OUT", default_out))
    dest.write_text(json.dumps(out, indent=2))
    print(f"\nwrote {dest}")


if __name__ == "__main__":
    main()
