#!/usr/bin/env python3
"""Replay CodeTether's exact OpenRouter body and isolate rejected tool schemas.

The control request with a simple tool succeeds, while the real 82-tool request
returns HTTP 400. This captures the body from CodeTether's debug log in memory,
replays it directly, and bisects tools to distinguish an invalid schema from a
count/size interaction. The API key is read from the environment and never
printed.
"""

import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request

URL = "https://openrouter.ai/api/v1/chat/completions"
GEMMA = "google/gemma-4-26b-a4b-it:free"
SAFETY = "nvidia/nemotron-3.5-content-safety:free"
ANSI = re.compile(r"\x1b\[[0-9;]*m")
CODING = {
    "agent", "apply_patch", "browserctl", "close_agent", "codesearch",
    "computer_use", "create_goal", "exec_command", "followup_task", "glob",
    "grep", "get_goal", "image", "image_gen", "interrupt_agent", "list",
    "list_agents", "lsp", "mux_control", "read", "resume_agent", "send_input",
    "send_message", "session_task", "skill", "spawn_agent", "update_goal",
    "wait_agent", "webfetch", "websearch", "write_stdin",
}


def tool_name(tool: dict) -> str:
    """Return one OpenAI function tool's name."""
    return tool.get("function", {}).get("name", "")


def key() -> str:
    """Return the API key without logging it."""
    value = os.environ.get("OPENROUTER_API_KEY", "").strip()
    if not value:
        raise RuntimeError("set OPENROUTER_API_KEY")
    return value


def capture_body() -> dict:
    """Capture the real streaming request body from a local CodeTether run."""
    env = dict(os.environ)
    env["RUST_LOG"] = "codetether_agent::provider::openrouter=debug"
    run = subprocess.run(
        [
            "/home/riley/.cargo/bin/codetether",
            "run",
            "--model",
            f"openrouter/{GEMMA}",
            "hello",
        ],
        capture_output=True,
        text=True,
        timeout=180,
        env=env,
        check=False,
    )
    output = ANSI.sub("", run.stdout + run.stderr)
    line = next(
        item for item in output.splitlines() if "Starting streaming request" in item
    )
    encoded = line.split("body=", 1)[1]
    return json.JSONDecoder().raw_decode(encoded)[0]


def post(body: dict, label: str) -> tuple[int, dict | None]:
    """Post one exact request variant and print its full error object."""
    request = urllib.request.Request(
        URL,
        data=json.dumps(body).encode(),
        headers={
            "Authorization": f"Bearer {key()}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://codetether.run",
            "X-Title": "CodeTether Agent",
        },
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            response.read(256)
            print(f"{label:35} 200")
            return 200, None
    except urllib.error.HTTPError as error:
        raw = error.read().decode("utf-8", "replace")
        try:
            detail = json.loads(raw).get("error")
        except json.JSONDecodeError:
            detail = {"raw": raw[:500]}
        print(f"{label:35} {error.code} {json.dumps(detail)[:700]}")
        return error.code, detail


def variant(base: dict, tools: list[dict], model: str = GEMMA) -> dict:
    """Clone the captured request with a chosen model and tool subset."""
    body = dict(base)
    body["model"] = model
    if tools:
        body["tools"] = tools
    else:
        body.pop("tools", None)
    return body


def bisect_failure(base: dict, tools: list[dict]) -> None:
    """Descend into a failing tool subset, or report a combination failure."""
    current = tools
    while len(current) > 1:
        middle = len(current) // 2
        left, right = current[:middle], current[middle:]
        left_status, _ = post(variant(base, left), f"left {len(left)} tools")
        if left_status != 200:
            current = left
            continue
        right_status, _ = post(variant(base, right), f"right {len(right)} tools")
        if right_status != 200:
            current = right
            continue
        print("Both halves pass: failure depends on combined count/size")
        return
    name = tool_name(current[0]) or "<unknown>"
    print(f"isolated failing schema: {name}")


def main() -> int:
    """Capture, replay, and bisect the real request."""
    body = capture_body()
    tools = body.get("tools", [])
    print(f"captured bytes={len(json.dumps(body))} tools={len(tools)}")
    post(variant(body, []), "Gemma without tools")
    full_status, _ = post(variant(body, tools), "Gemma full catalog")
    post(variant(body, [], SAFETY), "Safety without tools")
    compact = [tool for tool in tools if tool_name(tool) in CODING]
    compact_status, _ = post(variant(body, compact), "Gemma compact catalog")
    print("compact names:", ",".join(map(tool_name, compact)))
    if full_status != 200:
        bisect_failure(body, tools)
    return 0


if __name__ == "__main__":
    sys.exit(main())