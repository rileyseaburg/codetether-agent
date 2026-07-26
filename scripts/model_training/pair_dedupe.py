"""Remove train rows that duplicate any validation example."""

import json

from pathlib import Path


def prune(train: Path, validation: Path) -> dict[str, object]:
    """Drop exact prompt+completion duplicates from the training file."""
    held = {_key(json.loads(line)) for line in _lines(validation)}
    kept: list[str] = []
    removed = 0
    for line in _lines(train):
        if _key(json.loads(line)) in held:
            removed += 1
            continue
        kept.append(line)
    train.write_text(''.join(f'{line}\n' for line in kept))
    return {
        'removed_train_duplicates': removed,
        'train_records': len(kept),
        'validation_records': len(held),
        'exact_overlap': 0,
    }


def _lines(path: Path) -> list[str]:
    return path.read_text().splitlines()


def _key(record: dict[str, object]) -> str:
    return f'{record["prompt"]}{record["completion"]}'
