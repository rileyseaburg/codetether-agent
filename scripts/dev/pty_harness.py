#!/usr/bin/env python3
"""Shared PTY helpers for driving a real CodeTether TUI in tests.

The TUI blocks on a cursor-position report (``ESC[6n``) and ratatui refuses
to draw into a zero-sized PTY, so any harness must answer DSR and set a
window size before expecting frames.
"""

import contextlib
import fcntl
import os
import pathlib
import re
import select
import struct
import subprocess
import termios
import time


ESC = b"\x1b"
PASTE_START = ESC + b"[200~"
PASTE_END = ESC + b"[201~"
CTRL_V = b"\x16"

_OSC = re.compile(r"\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)")
_CSI = re.compile(r"\x1b\[[0-9;?]*[A-Za-z]")
_ESC_SEQ = re.compile(r"\x1b[>=()][0-9A-Za-z]?")
_NON_PRINT = re.compile(r"[^\x20-\x7e\n]")


def set_window(fd: int, rows: int = 40, cols: int = 120) -> None:
    """Give the PTY a real size so the TUI renders frames."""
    packed = struct.pack("HHHH", rows, cols, 0, 0)
    fcntl.ioctl(fd, termios.TIOCSWINSZ, packed)


def drain(fd: int, seconds: float) -> bytes:
    """Read for ``seconds``, answering terminal capability queries."""
    out = b""
    deadline = time.time() + seconds
    while time.time() < deadline:
        if not select.select([fd], [], [], 0.2)[0]:
            continue
        try:
            data = os.read(fd, 65536)
        except OSError:
            break
        if not data:
            break
        out += data
        if ESC + b"[6n" in data:
            os.write(fd, ESC + b"[24;1R")
        if ESC + b"[>1u" in data:
            os.write(fd, ESC + b"[?0u")
    return out


def visible(raw: bytes) -> str:
    """Strip escape sequences so screen text can be grepped."""
    text = raw.decode("utf-8", "replace")
    for pattern in (_OSC, _CSI, _ESC_SEQ):
        text = pattern.sub("", text)
    return _NON_PRINT.sub("", text)


def shutdown(fd: int, pid: int) -> None:
    """Close the PTY and hard-kill the child."""
    with contextlib.suppress(OSError):
        os.write(fd, b"\x03")
        time.sleep(0.3)
        os.close(fd)
    subprocess.run(["kill", "-9", str(pid)], check=False)


def report(out_path: str, raw: bytes, needles: tuple[str, ...]) -> str:
    """Save decoded screen text and print a hit summary for each needle."""
    text = visible(raw)
    with pathlib.Path(out_path).open("w", encoding="utf-8") as handle:
        handle.write(text)
    print(f"=== log: {out_path} ({len(raw)} raw bytes) ===")
    for needle in needles:
        pattern = r".{0,60}" + re.escape(needle) + r".{0,60}"
        hits = re.findall(pattern, text)
        print(f"[{needle}] {len(hits)} hit(s)")
        for hit in hits[:2]:
            print(f"    {hit}")
    return text
