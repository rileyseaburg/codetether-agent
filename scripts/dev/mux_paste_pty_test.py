#!/usr/bin/env python3
"""Send a real bracketed paste through `codetether mux attach` into a TUI.

Reproduces the user-facing path: local terminal -> mux client -> mux server
-> TUI. Compare against bracketed_paste_pty_test.py to isolate whether the
mux input proxy is what drops image pastes.

Usage:
    mux_paste_pty_test.py <session> <payload-file> [--out FILE]
"""

import argparse
import os
import pathlib
import pty
import sys

from pty_harness import (
    PASTE_END,
    PASTE_START,
    drain,
    report,
    set_window,
    shutdown,
)


NEEDLES = ("Attached pasted image", "Attached", "Pasted", "data:image")


def parse_args() -> argparse.Namespace:
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser()
    parser.add_argument("session")
    parser.add_argument("payload")
    parser.add_argument("--out", default="/tmp/ct-image-test/mux-pty.log")
    return parser.parse_args()


def main() -> int:
    """Attach to a mux session, start the TUI, and paste the payload."""
    args = parse_args()
    payload = pathlib.Path(args.payload).read_bytes().strip()

    pid, fd = pty.fork()
    if pid == 0:
        os.environ["TERM"] = "xterm-256color"
        os.execvp("codetether", ["codetether", "mux", "attach", args.session])
        os._exit(1)

    set_window(fd)
    raw = drain(fd, 12.0)
    os.write(fd, b"codetether tui --access-mode full\r")
    raw += drain(fd, 30.0)
    os.write(fd, PASTE_START + payload + PASTE_END)
    raw += drain(fd, 12.0)
    shutdown(fd, pid)

    report(args.out, raw, NEEDLES)
    return 0


if __name__ == "__main__":
    sys.exit(main())
