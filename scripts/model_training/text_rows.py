"""Read and write rendered SFT rows without changing their bytes."""

import hashlib
import json

from pathlib import Path


Row = dict[str, object]


def read(path: Path) -> list[Row]:
    """Decode a rendered JSONL dataset."""
    return [json.loads(line) for line in path.read_text().splitlines()]


def sha(row: Row) -> str:
    """Return the governed semantic message digest."""
    metadata = row['metadata']
    if not isinstance(metadata, dict):
        raise ValueError('metadata must be an object')
    return str(metadata['message_sha256'])


def correlation(row: Row) -> str:
    """Return the source conversation identifier."""
    metadata = row['metadata']
    if not isinstance(metadata, dict):
        raise ValueError('metadata must be an object')
    return str(metadata['correlation_id'])


def unique(rows: list[Row], excluded: set[str]) -> list[Row]:
    """Return the first row for each digest not already excluded."""
    selected: list[Row] = []
    for row in rows:
        digest = sha(row)
        if digest not in excluded:
            selected.append(row)
            excluded.add(digest)
    return selected


def write(path: Path, rows: list[Row]) -> str:
    """Serialize rows deterministically and return their SHA-256."""
    payload = ''.join(
        json.dumps(row, sort_keys=True, separators=(',', ':')) + '\n'
        for row in rows
    )
    path.write_text(payload)
    return hashlib.sha256(payload.encode()).hexdigest()
