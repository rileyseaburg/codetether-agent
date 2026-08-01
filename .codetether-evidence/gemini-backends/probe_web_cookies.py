#!/usr/bin/env python3
"""Probe the cookie-authenticated gemini.google.com BardChatUi backend.

This is the transport CodeTether's existing `gemini-web` provider uses. It is
undocumented: auth is browser cookies and the response is positional JSON
frames, so the answer text lives at a fixed slot path that Google may move.

Credentials are read from Vault (never hardcoded, never printed):
    vault kv get -field=cookies secret/codetether/providers/gemini-web

Usage:
    probe_web_cookies.py "prompt text" [--json]
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import urllib.parse

import requests

ORIGIN = "https://gemini.google.com"
STREAM_PATH = "/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate"
MODE_ID_FAST = "fbb127bbb056c959"


def load_cookies() -> str:
    """Load the cookies.txt blob from Vault without echoing it."""
    out = subprocess.run(
        ["vault", "kv", "get", "-field=cookies",
         "secret/codetether/providers/gemini-web"],
        capture_output=True, text=True, check=True,
    )
    return out.stdout


def cookie_header(raw: str) -> str:
    pairs = []
    for line in raw.splitlines():
        line = line.strip()
        if not line or line.startswith("# ") or line == "#":
            continue
        line = line.removeprefix("#HttpOnly_")
        cols = line.split("\t")
        if len(cols) >= 7:
            pairs.append(f"{cols[5]}={cols[6]}")
        elif len(cols) == 2:
            pairs.append(f"{cols[0]}={cols[1]}")
    return "; ".join(pairs)


def session_tokens(session: requests.Session) -> dict[str, str]:
    """Scrape at/bl/f_sid tokens from the app shell, as the web client does."""
    home = session.get(ORIGIN, timeout=30)
    home.raise_for_status()
    body = home.text

    def find(pattern: str, required: bool = True) -> str:
        match = re.search(pattern, body)
        if match:
            return match.group(1)
        if required:
            raise RuntimeError(f"token scrape failed for {pattern!r}")
        return ""

    # The XSRF token key is unstable: it was "SNlM0e" and is now "thykhd".
    # That drift is exactly the fragility this probe is meant to document.
    at_token = find(r'"thykhd":"([^"]+)"', required=False) or find(
        r'"SNlM0e":"([^"]+)"'
    )

    return {
        "at": at_token,
        "bl": find(r'"cfb2h":"([^"]+)"'),
        "f_sid": find(r'"FdrFJe":"([^"]+)"', required=False),
    }


def extract_text(raw: str) -> str | None:
    """Read the answer from the positional slot the web client uses."""
    latest = None
    for line in raw.splitlines():
        line = line.strip()
        if not line.startswith("["):
            continue
        try:
            events = json.loads(line)
        except json.JSONDecodeError:
            continue
        for event in events if isinstance(events, list) else []:
            try:
                inner = json.loads(event[2])
                candidate = inner[4][0][1][0]
            except (IndexError, TypeError, ValueError, json.JSONDecodeError):
                continue
            if isinstance(candidate, str) and candidate:
                latest = candidate
    return latest


def probe(prompt: str) -> dict:
    raw_cookies = load_cookies()
    session = requests.Session()
    session.headers.update({
        "User-Agent": (
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 "
            "(KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
        ),
        "Cookie": cookie_header(raw_cookies),
        "Origin": ORIGIN,
        "Referer": f"{ORIGIN}/app",
    })

    tokens = session_tokens(session)

    # Nested-array positional payload: the prompt is buried at f.req[1] as a
    # JSON string inside a JSON string. There is no named field for it.
    inner = json.dumps([[prompt], None, None])
    form = {"f.req": json.dumps([None, inner]), "at": tokens["at"]}
    params = {
        "bl": tokens["bl"],
        "f.sid": tokens["f_sid"],
        "_reqid": "1000",
        "rt": "c",
    }

    response = session.post(
        f"{ORIGIN}{STREAM_PATH}?{urllib.parse.urlencode(params)}",
        data=form,
        timeout=180,
    )

    body = response.text
    return {
        "backend": "gemini.google.com BardChatUi (cookies)",
        "endpoint": f"{ORIGIN}{STREAM_PATH}",
        "auth": "browser cookies from Vault",
        "model_selector": f"x-goog-ext mode_id {MODE_ID_FAST}",
        "http_status": response.status_code,
        "response_bytes": len(body),
        "answer_text": extract_text(body),
        "structured_tool_calls": False,
        "reported_token_usage": False,
        "notes": [
            "answer parsed from positional slot [4][0][1][0]",
            "tool calls only exist as <tool_call> text the model may forge",
            "no usage/token accounting in the payload",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("prompt")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()

    try:
        result = probe(args.prompt)
    except Exception as error:  # surface the real failure, never invent success
        result = {"backend": "gemini.google.com BardChatUi (cookies)",
                  "error": f"{type(error).__name__}: {error}"}

    print(json.dumps(result, indent=2) if args.json else result)
    return 0 if "error" not in result else 1


if __name__ == "__main__":
    sys.exit(main())