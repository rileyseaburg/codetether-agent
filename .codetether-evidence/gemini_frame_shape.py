#!/usr/bin/env python3
"""Dump the Gemini Web wire-frame structure to find the answer vs. thinking slot.

Symptoms after the tool-format fix:
  - assistant output contains raw model reasoning ("Let's respond warmly...",
    "RULE 1: STRICT COMPLETION")
  - a turn sometimes surfaces Gemini's own "Sorry, something went wrong."
    (that string does not exist anywhere in this repo)

`response_text::event_text` reads exactly one slot, [4][0][1][0]. For thinking
models that slot can hold reasoning instead of the final answer. This prints the
shape of every candidate slot per frame so the correct index is chosen from
evidence rather than guessed.

Requires a captured body. Set GEMINI_BODY to a file containing the raw
StreamGenerate response.

Usage:
    GEMINI_BODY=/tmp/gemini_body.txt python3 gemini_frame_shape.py
"""

import json
import os
import pathlib
import sys


def frames(raw: str):
    """Yield parsed top-level frame arrays from a StreamGenerate body."""
    for line in raw.splitlines():
        line = line.strip()
        if not line.startswith("["):
            continue
        try:
            parsed = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(parsed, list):
            yield parsed


def describe(value, depth=0, path="") -> list[tuple[str, str]]:
    """Collect (path, preview) for every string leaf, bounded in depth."""
    found = []
    if depth > 6:
        return found
    if isinstance(value, str):
        if value.strip():
            preview = value.replace("\n", "\\n")[:110]
            found.append((path or "<root>", preview))
        return found
    if isinstance(value, list):
        for index, item in enumerate(value):
            found.extend(describe(item, depth + 1, f"{path}[{index}]"))
    return found


def main():
    body_path = os.environ.get("GEMINI_BODY")
    if not body_path or not pathlib.Path(body_path).is_file():
        print("set GEMINI_BODY to a captured StreamGenerate body", file=sys.stderr)
        return 1
    raw = pathlib.Path(body_path).read_text(errors="replace")

    for frame_index, frame in enumerate(frames(raw)):
        for event in frame:
            if not isinstance(event, list) or len(event) < 3:
                continue
            inner = event[2]
            if not isinstance(inner, str) or not inner.startswith("["):
                continue
            try:
                payload = json.loads(inner)
            except json.JSONDecodeError:
                continue
            leaves = describe(payload)
            if not leaves:
                continue
            print(f"=== frame {frame_index} ===")
            for path, preview in leaves[:14]:
                marker = "  <-- current slot" if path == "[4][0][1][0]" else ""
                print(f"  {path:22} {preview}{marker}")
            print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
