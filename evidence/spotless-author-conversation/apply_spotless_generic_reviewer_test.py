"""Align reviewer tests with ordered generic candidate selection."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/tests/codetether-review-approval-credentials.test.sh")
OLD = "run_case riley spotless-agent kv/forgejo/spotlessbinco-agent\n"
NEW = "run_case riley codetether-bot kv/forgejo/codetether-bot\n"


def main() -> None:
    """Replace the repository-specific fallback expectation."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("reviewer expectation anchor missing or ambiguous")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()