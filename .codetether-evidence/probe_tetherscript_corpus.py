#!/usr/bin/env python3
"""Check every checked-in TetherScript plugin for false-positive diagnostics.

Wiring the TetherScript LSP into the post-edit verification hook only helps if it
is quiet on code that is already correct. This runs `tetherscript lsp` over every
`examples/tetherscript/*.tether` file and reports any that produce diagnostics.

Usage:
    python3 probe_tetherscript_corpus.py
"""

import glob
import json
import subprocess
import sys
import threading


def frame(obj):
    """Encode a JSON-RPC message with LSP Content-Length framing."""
    body = json.dumps(obj).encode()
    return b"Content-Length: " + str(len(body)).encode() + b"\r\n\r\n" + body


def read_messages(stream, limit):
    """Read up to `limit` framed JSON-RPC messages."""
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
            pass
    return seen


def diagnostics_for(path):
    """Return published diagnostics for one plugin file."""
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    proc = subprocess.Popen(
        ["tetherscript", "lsp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    timer = threading.Timer(15.0, proc.kill)
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
            if message.get("method") == "textDocument/publishDiagnostics":
                return message["params"].get("diagnostics", [])
        return []
    finally:
        timer.cancel()
        proc.kill()


def main():
    """Report plugins that produce diagnostics."""
    paths = sorted(glob.glob("examples/tetherscript/*.tether"))
    if not paths:
        print("no plugins found", file=sys.stderr)
        return 1
    noisy = {}
    for path in paths:
        if path.endswith("_lsp_smoke_broken.tether"):
            continue
        diags = diagnostics_for(path)
        if diags:
            noisy[path] = diags
    print(f"scanned {len(paths)} plugins; {len(noisy)} produced diagnostics")
    for path, diags in noisy.items():
        first = diags[0]
        line = first.get("range", {}).get("start", {}).get("line")
        print(f"  {path}: line {line}: {first.get('message')}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
