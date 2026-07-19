"""Run the repository formatter on the split Playwright contract modules."""

import subprocess
from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
FILES = sorted(
    str(path.relative_to(ROOT))
    for path in (ROOT / "tests/playwright/codetether").glob("pr-review-*.ts")
)


def main() -> None:
    """Format only protocol-related TypeScript files."""
    subprocess.run(
        ["npx", "prettier", "--write", *FILES],
        cwd=ROOT,
        check=True,
    )


if __name__ == "__main__":
    main()