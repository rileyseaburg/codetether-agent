#!/usr/bin/env python3
"""Verify `tetherscript lsp` speaks LSP and publishes real diagnostics.

Confirms, before wiring it into the harness, that:
  1. `initialize` returns capabilities and serverInfo.
  2. `textDocument/didOpen` on a broken file yields publishDiagnostics.
  3. The diagnostic carries a usable range and message.

Usage:
    python3 probe_tetherscript_lsp.py /tmp/probe_bad.tether
"""

import json
import subprocess
import sys
import threading


def frame(obj):
    """Encode a JSON-RPC message with LSP Content-Length framing."""
    body = json.dumps(obj).encode()
    return b"Content-Length: " + str(len(body)).encode() + b"\r\n\r\n" + body


def read_messages(stream, limit=8):
    """Read up to `limit` framed JSON-RPC messages from `stream`."""
    seen = []
    while len(seen) < limit:
        header = b""
        while b"\r\n\r\n" not in header:
            byte = stream.read(1)
            if not byte:
                return seen
            header += byte
        length = 0
        for line in header.decode("utf-8", "replace").split("\r\n"):
            if line.lower().startswith("content-length:"):
                length = int(line.split(":", 1)[1].strip())
        payload = stream.read(length) if length else b"{}"
        try:
            seen.append(json.loads(payload))
        except json.JSONDecodeError:
            seen.append({"raw": payload.decode("utf-8", "replace")})
    return seen


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else "/tmp/probe_bad.tether"
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    proc = subprocess.Popen(
        ["tetherscript", "lsp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    timer = threading.Timer(8.0, proc.kill)
    timer.start()
    try:
        for msg in (
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"rootUri": None, "capabilities": {}},
            },
            {"jsonrpc": "2.0", "method": "initialized", "params": {}},
            {
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": {
                        "uri": f"file://{path}",
                        "languageId": "tetherscript",
                        "version": 1,
                        "text": text,
                    }
                },
            },
        ):
            proc.stdin.write(frame(msg))
        proc.stdin.flush()
        for message in read_messages(proc.stdout, limit=3):
            print(json.dumps(message, indent=2)[:900], flush=True)
    finally:
        timer.cancel()
        proc.kill()


if __name__ == "__main__":
    main()
