"""Deterministic JSONL serialization and dataset hashing."""

import hashlib
import json

from pathlib import Path


def write(path: Path, rows: list[dict[str, object]]) -> dict[str, object]:
    """Atomically write trainer records and return immutable metadata."""
    digest = hashlib.sha256()
    correlations: set[str] = set()
    temporary = path.with_suffix('.jsonl.tmp')
    with temporary.open('wb') as output:
        for row in rows:
            correlation = str(row['correlation_id'])
            correlations.add(correlation)
            record = {
                'messages': json.loads(str(row['messages_json'])),
                'metadata': {
                    'sample_id': row['sample_id'],
                    'correlation_id': correlation,
                    'message_sha256': row['message_sha'],
                },
            }
            line = _line(record)
            digest.update(line)
            output.write(line)
    temporary.replace(path)
    return {
        'path': str(path),
        'records': len(rows),
        'correlations': len(correlations),
        'sha256': digest.hexdigest(),
        'bytes': path.stat().st_size,
    }


def _line(value: dict[str, object]) -> bytes:
    encoded = json.dumps(value, sort_keys=True, separators=(',', ':'))
    return encoded.encode() + b'\n'
