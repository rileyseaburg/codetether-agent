#!/usr/bin/env python3
"""Show what the Gemini Web prompt actually teaches the model.

The provider parses `<tool_call>{"name":...,"arguments":{...}}</tool_call>`, but
the protocol rules only *mention* `<tool_call>` blocks without ever showing the
JSON shape or a concrete example. This prints the rules block so the gap is
visible in review.

Usage:
    python3 gemini_prompt_gap.py
"""

import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
PROTOCOL = ROOT / "src/provider/gemini_web/prompt/protocol.rs"
PARSE = ROOT / "src/provider/gemini_web/tool_calls/parse.rs"


def concat_literals(source: str, const_name: str) -> str:
    """Extract and join the string literals of a `concat!` constant."""
    match = re.search(
        rf"{const_name}: &str = concat!\((.*?)\);", source, re.S
    )
    if not match:
        return ""
    return "".join(re.findall(r'"((?:[^"\\]|\\.)*)"', match.group(1)))


def main():
    protocol = PROTOCOL.read_text()
    rules = concat_literals(protocol, "RULES").replace("\\n", "\n")
    print("=== RULES sent to the model ===")
    print(rules)
    print()

    needles = ['"name"', '"arguments"', "{", "example"]
    print("=== does the prompt show the required JSON shape? ===")
    for needle in needles:
        present = needle in rules
        print(f"  {needle!r:14} present={present}")

    parse = PARSE.read_text()
    print()
    print("=== fields the parser requires ===")
    for field in re.findall(r'value\.get\("(\w+)"\)', parse):
        print(f"  requires: {field}")


if __name__ == "__main__":
    main()
