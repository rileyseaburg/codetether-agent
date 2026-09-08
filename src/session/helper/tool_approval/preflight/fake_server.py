"""Minimal LSP process fixture for automatic preflight deadline tests."""

import json
import sys
import time


def send(message: dict) -> None:
    body = json.dumps({"jsonrpc": "2.0", **message}).encode()
    sys.stdout.buffer.write(f"Content-Length: {len(body)}\r\n\r\n".encode() + body)
    sys.stdout.buffer.flush()


mode = sys.argv[1]
while True:
    length = 0
    while line := sys.stdin.buffer.readline():
        if line in (b"\r\n", b"\n"):
            break
        if line.lower().startswith(b"content-length:"):
            length = int(line.split(b":", 1)[1])
    if not length:
        break
    message = json.loads(sys.stdin.buffer.read(length))
    method = message.get("method")
    if method == "initialize":
        if mode == "slow_init":
            time.sleep(0.4)
            mode = "healthy"
        if mode == "startup_hang":
            time.sleep(60)
        send({"id": message["id"], "result": {"capabilities": {}}})
    elif method == "shutdown":
        send({"id": message["id"], "result": None})
    elif method == "exit":
        break
    elif method in ("textDocument/didOpen", "textDocument/didChange") and mode in ("healthy", "slow"):
        if mode == "slow":
            time.sleep(5.4)
            mode = "healthy"
        send({"method": "textDocument/publishDiagnostics", "params": {
            "uri": message["params"]["textDocument"]["uri"], "diagnostics": []}})