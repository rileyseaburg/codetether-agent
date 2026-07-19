"""Make the source-contract assertion follow the approval module split."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/tests/playwright/codetether/pr-review-workflow.spec.ts")
OLD = '''    const approval = await readFile("scripts/codetether-review/approval.sh", "utf8");
'''
NEW = '''    const approval = (
      await Promise.all(
        ["approval_input.sh", "approval.sh"].map((name) =>
          readFile(`scripts/codetether-review/${name}`, "utf8"),
        ),
      )
    ).join("\\n");
'''


def main() -> None:
    """Replace the monolithic approval source read exactly once."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("Playwright approval source anchor is missing")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()