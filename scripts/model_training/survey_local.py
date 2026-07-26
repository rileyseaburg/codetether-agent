"""Survey on-disk agent transcripts to size the real training corpus."""

import argparse
import collections
import json

from pathlib import Path

from .transcript_roles import is_assistant


def main() -> None:
    """Count transcript files, records, and assistant turns per source."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--roots', nargs='+', required=True)
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    report: dict[str, object] = {}
    for root in values.roots:
        report[root] = _survey(Path(root))
    values.output.write_text(
        json.dumps(report, indent=2, sort_keys=True) + '\n'
    )
    print(json.dumps(report, sort_keys=True))


def _survey(root: Path) -> dict[str, object]:
    kinds: collections.Counter[str] = collections.Counter()
    files = records = assistants = size = 0
    for path in root.rglob('*.jsonl'):
        files += 1
        size += path.stat().st_size
        for line in path.open(errors='ignore'):
            record = _decode(line)
            if record is None:
                continue
            records += 1
            kind = str(record.get('type') or record.get('role') or 'unknown')
            kinds[kind] += 1
            if is_assistant(record):
                assistants += 1
    return {
        'files': files,
        'bytes': size,
        'records': records,
        'assistant_turns': assistants,
        'kinds': dict(kinds.most_common(12)),
    }


def _decode(line: str) -> dict[str, object] | None:
    try:
        value = json.loads(line)
    except ValueError:
        return None
    return value if isinstance(value, dict) else None


if __name__ == '__main__':
    main()
