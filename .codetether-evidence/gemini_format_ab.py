#!/usr/bin/env python3
"""A/B whether showing the tool-call format makes Gemini Web emit real calls.

Users report non-stop tool failure on gemini-web models. Live runs show the
model narrating tool use in prose ("I am calling the list tool...") while
emitting zero parseable `<tool_call>` blocks.

Hypothesis: the prompt lists 14 prohibitions about `<tool_call>` blocks but never
demonstrates the required JSON shape, so the model never learns the syntax the
parser requires.

This sends both prompt variants through the real Gemini Web endpoint via the
installed `codetether` binary is NOT used here; instead we reuse the provider's
own session by shelling out to `codetether run` with a crafted user message that
carries the example inline. That isolates the prompt variable without a rebuild.

Usage:
    python3 gemini_format_ab.py [trials]
"""

import json
import pathlib
import re
import subprocess
import sys

BINARY = "/home/riley/.cargo/bin/codetether"
MODEL = "gemini-web/gemini-web-fast"
TASK = 'Call the list tool with path "." now.'

# The exact shape src/provider/gemini_web/tool_calls/parse.rs requires.
EXAMPLE = (
    "Emit tool calls in exactly this raw format, one block per call:\n"
    '<tool_call>{"name": "list", "arguments": {"path": "."}}</tool_call>\n'
    "The block must contain a JSON object with a `name` string and an "
    "`arguments` object. Do not describe the call in prose."
)

CALL_RE = re.compile(r"<tool_call>", re.I)


def run(message: str, timeout: int = 220) -> str:
    """Run one non-interactive turn and return combined output."""
    proc = subprocess.run(
        [BINARY, "run", "--model", MODEL, message],
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    return proc.stdout + proc.stderr


def narrates_without_calling(output: str) -> bool:
    """True when the model talks about tools instead of invoking them."""
    prose = re.search(
        r"I (am|will) (calling|call|use|using|run|running|execute)", output, re.I
    )
    return bool(prose)


def classify(output: str) -> str:
    """Label one turn's outcome."""
    if "Tool call feedback" in output or "list" in output and "\n- tool:" in output:
        return "tool_executed"
    if CALL_RE.search(output):
        return "emitted_raw_markup"
    if narrates_without_calling(output):
        return "narrated_only"
    return "no_tool_attempt"


def main():
    trials = int(sys.argv[1]) if len(sys.argv) > 1 else 3
    results = {"without_example": [], "with_example": []}
    for index in range(trials):
        plain = run(TASK)
        results["without_example"].append(classify(plain))
        print(f"[{index + 1}] without_example -> {results['without_example'][-1]}", flush=True)

        guided = run(f"{EXAMPLE}\n\n{TASK}")
        results["with_example"].append(classify(guided))
        print(f"[{index + 1}] with_example    -> {results['with_example'][-1]}", flush=True)

    print("\n=== tally ===")
    for variant, outcomes in results.items():
        counts = {}
        for outcome in outcomes:
            counts[outcome] = counts.get(outcome, 0) + 1
        print(f"{variant:16} {counts}")

    dest = pathlib.Path("/tmp/gemini_format_ab.json")
    dest.write_text(json.dumps(results, indent=2))
    print(f"\nwrote {dest}")


if __name__ == "__main__":
    main()
