"""Split remaining oversized review helpers into cohesive modules."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
DIR = ROOT / "scripts/codetether-review"
MODULES = DIR / "modules.sh"
SOURCE = ROOT / "tests/playwright/codetether/pr-review-source.ts"


def insert(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"legacy split anchor missing: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def split_forgejo() -> None:
    """Separate comment upserts from the Forgejo transport/review reader."""
    source = DIR / "forgejo.sh"
    target = DIR / "forgejo_comments.sh"
    text = source.read_text()
    marker = '# Finds the first comment containing an exact workflow marker.\n'
    if text.count(marker) == 1:
        before, comments = text.split(marker, 1)
        source.write_text(before.rstrip() + '\n')
        target.write_text('#!/usr/bin/env bash\n\n' + marker + comments)
    elif not target.exists():
        raise RuntimeError('Forgejo comment helpers are missing')
    insert(MODULES, 'source "$review_module_dir/forgejo.sh"\n',
        'source "$review_module_dir/forgejo_comments.sh"\n')
    insert(SOURCE, '  "scripts/codetether-review/forgejo.sh",\n',
        '  "scripts/codetether-review/forgejo_comments.sh",\n')


def split_payload() -> None:
    """Separate prompt, input, async, and sync payload responsibilities."""
    source = DIR / "payload.sh"
    text = source.read_text()
    markers = [
        '# Builds the repository and diff input consumed by either CodeTether endpoint.\n',
        '# Wraps review input in the asynchronous agent-task request contract.\n',
        '# Wraps review input in the synchronous structured-JSON request contract.\n',
    ]
    if all(text.count(marker) == 1 for marker in markers):
        prompt, rest = text.split(markers[0], 1)
        input_part, rest = rest.split(markers[1], 1)
        async_part, sync_part = rest.split(markers[2], 1)
        parts = {
            'review_prompt.sh': prompt,
            'payload_input.sh': '#!/usr/bin/env bash\n\n' + markers[0] + input_part,
            'payload_async.sh': '#!/usr/bin/env bash\n\n' + markers[1] + async_part,
            'payload_sync.sh': '#!/usr/bin/env bash\n\n' + markers[2] + sync_part,
        }
        for name, content in parts.items():
            (DIR / name).write_text(content.rstrip() + '\n')
        source.write_text('''#!/usr/bin/env bash

payload_module_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$payload_module_dir/review_prompt.sh"
source "$payload_module_dir/payload_input.sh"
source "$payload_module_dir/payload_async.sh"
source "$payload_module_dir/payload_sync.sh"
unset payload_module_dir
''')
    elif not (DIR / 'payload_async.sh').exists():
        raise RuntimeError('payload helper split is incomplete')
    anchor = '  "scripts/codetether-review/payload.sh",\n'
    addition = ''.join(
        f'  "scripts/codetether-review/{name}",\n'
        for name in ('review_prompt.sh', 'payload_input.sh', 'payload_async.sh', 'payload_sync.sh')
    )
    insert(SOURCE, anchor, addition)


def main() -> None:
    """Apply both independent legacy splits."""
    split_forgejo()
    split_payload()


if __name__ == "__main__":
    main()