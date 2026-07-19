"""Split the growing shell path manifest from source readers."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco/tests/playwright/codetether")
SOURCE = ROOT / "pr-review-source.ts"
PATHS = ROOT / "pr-review-script-paths.ts"


def main() -> None:
    """Move the array while preserving its public import contract."""
    text = SOURCE.read_text()
    marker = 'export const scriptPaths = ['
    if marker in text:
        start = text.index(marker)
        end = text.index('];', start) + 2
        array = text[start:end]
        PATHS.write_text(array + '\n')
        prefix = '''import { readFile } from "node:fs/promises";
import { scriptPaths } from "./pr-review-script-paths";

export { scriptPaths } from "./pr-review-script-paths";

'''
        suffix = text[end:].lstrip()
        SOURCE.write_text(prefix + suffix)
    elif not PATHS.exists():
        raise RuntimeError('Playwright shell path manifest is missing')


if __name__ == "__main__":
    main()