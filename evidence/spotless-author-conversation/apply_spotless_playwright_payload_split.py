"""Point the payload contract assertion at all focused payload modules."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/tests/playwright/codetether/pr-review-security.cases.ts")
OLD = '''    const payload = await readFile("scripts/codetether-review/payload.sh", "utf8");
    const response = await readFile("scripts/codetether-review/response.sh", "utf8");
    const approval = await readScripts();
'''
NEW = '''    const payload = await readScripts();
    const response = await readFile("scripts/codetether-review/response.sh", "utf8");
    const approval = payload;
'''


def main() -> None:
    """Replace the obsolete monolithic payload read."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("Playwright payload assertion anchor is missing")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()