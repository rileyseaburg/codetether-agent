#!/usr/bin/env python3
"""Send a real bracketed paste to a directly launched CodeTether TUI.

Writes genuine ESC bytes, so the TUI receives an actual
``ESC[200~ ... ESC[201~`` paste rather than literal bracket characters.
With ``--ctrl-v`` it instead sends Ctrl+V to exercise the server-side
clipboard read, which fails on a headless SSH host.

Usage:
    bracketed_paste_pty_test.py <payload-file> [--ctrl-v] [--out FILE]
"""

import argparse
import os
import pathlib
import pty
import sys

from pty_harness import (
    CTRL_V,
    PASTE_END,
    PASTE_START,
    drain,
    report,
    set_window,
    shutdown,
)


NEEDLES = ("Attached pasted image", "Attached", "Pasted", "clipboard")


def parse_args() -> argparse.Namespace:
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser()
    parser.add_argument("payload")
    parser.add_argument("--ctrl-v", action="store_true")
    parser.add_argument("--out", default="/tmp/ct-image-test/pty.log")
    return parser.parse_args()


def main() -> int:
    """Launch the TUI, deliver one paste, and summarise the screen."""
    args = parse_args()
    payload = pathlib.Path(args.payload).read_bytes().strip()

    pid, fd = pty.fork()
    if pid == 0:
        os.environ["TERM"] = "xterm-256color"
        os.execvp("codetether", ["codetether", "tui", "--access-mode", "full"])
        os._exit(1)

    set_window(fd)
    raw = drain(fd, 22.0)
    os.write(fd, CTRL_V if args.ctrl_v else PASTE_START + payload + PASTE_END)
    raw += drain(fd, 10.0)
    os.write(fd, b"describe this image")
    raw += drain(fd, 6.0)
    shutdown(fd, pid)

    report(args.out, raw, NEEDLES)
    return 0


if __name__ == "__main__":
    sys.exit(main())
