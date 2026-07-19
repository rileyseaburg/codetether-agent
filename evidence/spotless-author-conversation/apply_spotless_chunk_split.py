"""Split legacy chunk aggregation to satisfy the global 50-line rule."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
CHUNKS = ROOT / "scripts/codetether-review/chunks.sh"
AGGREGATE = ROOT / "scripts/codetether-review/chunk_aggregate.sh"
MODULES = ROOT / "scripts/codetether-review/modules.sh"
TEST = ROOT / "tests/codetether-review-chunks.test.sh"
SOURCE = ROOT / "tests/playwright/codetether/pr-review-source.ts"


def insert(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"chunk split anchor missing: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Move the aggregation function and update every source manifest."""
    text = CHUNKS.read_text()
    marker = '# Produces one aggregate review and validates complete, ordered chunk coverage.\n'
    if text.count(marker) == 1:
        before, aggregate = text.split(marker, 1)
        CHUNKS.write_text(before.rstrip() + '\n')
        AGGREGATE.write_text('#!/usr/bin/env bash\n\n' + marker + aggregate)
    elif not AGGREGATE.exists():
        raise RuntimeError('chunk aggregation function is missing')
    insert(
        MODULES,
        'source "$review_module_dir/chunks.sh"\n',
        'source "$review_module_dir/chunk_aggregate.sh"\n',
    )
    insert(
        TEST,
        'source "$repo_root/scripts/codetether-review/chunks.sh"\n',
        'source "$repo_root/scripts/codetether-review/chunk_aggregate.sh"\n',
    )
    insert(
        SOURCE,
        '  "scripts/codetether-review/chunks.sh",\n',
        '  "scripts/codetether-review/chunk_aggregate.sh",\n',
    )


if __name__ == "__main__":
    main()