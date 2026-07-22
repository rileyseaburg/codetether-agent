#!/usr/bin/env python3
"""Convert strace text into lossless, structured JSON Lines."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import re
from pathlib import Path

from strace_lexer import lex

CALL = re.compile(r"^(?P<timestamp>\d+\.\d+) (?P<syscall>[A-Za-z_]\w*)\((?P<arguments>.*)\) = (?P<result>.*?)(?: <(?P<duration>\d+\.\d+)>)?$")
EXIT = re.compile(r"^(?P<timestamp>\d+\.\d+) \+\+\+ exited with (?P<exit_code>-?\d+) \+\+\+$")


def ending(line: bytes) -> tuple[bytes, str]:
    """Separate a record from its exact line terminator."""
    for marker, name in ((b"\r\n", "CRLF"), (b"\n", "LF"), (b"\r", "CR")):
        if line.endswith(marker):
            return line[: -len(marker)], name
    return line, "NONE"


def structure(text: str) -> dict[str, str | int | None]:
    """Extract syscall or exit fields while retaining unparsed text elsewhere."""
    if match := CALL.match(text):
        fields: dict[str, str | int | None] = {"record_type": "syscall", **match.groupdict()}
        return fields
    if match := EXIT.match(text):
        return {"record_type": "exit", "timestamp": match["timestamp"], "exit_code": int(match["exit_code"])}
    return {"record_type": "unclassified"}


def parse(source: Path, destination: Path) -> None:
    """Write one lossless JSON object per physical source line plus a manifest."""
    digest = hashlib.sha256()
    count = 0
    with source.open("rb") as incoming, destination.open("w", encoding="utf-8") as outgoing:
        for count, physical in enumerate(incoming, 1):
            digest.update(physical)
            content, terminator = ending(physical)
            text = content.decode("utf-8", errors="surrogateescape")
            record = {"line": count, "raw": text, "terminator": terminator, "raw_base64": base64.b64encode(physical).decode("ascii"), "tokens": [token.json() for token in lex(text)], "parsed": structure(text)}
            outgoing.write(json.dumps(record, ensure_ascii=True, separators=(",", ":")) + "\n")
    manifest = {"source": str(source), "destination": str(destination), "source_bytes": source.stat().st_size, "source_lines": count, "source_sha256": digest.hexdigest(), "encoding": "utf-8-surrogateescape", "lossless_field": "raw_base64"}
    Path(f"{destination}.manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("destination", type=Path)
    arguments = parser.parse_args()
    parse(arguments.source, arguments.destination)
