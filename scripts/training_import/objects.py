"""Build bounded immutable objects from a canonical session snapshot."""

import json

from pathlib import Path

from .records import session


MAX_BYTES = 8 * 1024 * 1024


def chunks(path: Path) -> list[tuple[str, bytes]]:
    """Return deterministic keys and bounded JSONL payloads."""
    value = json.loads(path.read_text())
    session_id = str(value.get('id') or path.stem)
    date_path = _date_path(value.get('created_at'))
    output: list[tuple[str, bytes]] = []
    lines: list[bytes] = []
    size = 0
    for record in session(value):
        line = _line(record)
        if lines and size + len(line) > MAX_BYTES:
            output.append(
                (_key(date_path, session_id, len(output)), b''.join(lines))
            )
            lines, size = [], 0
        lines.append(line)
        size += len(line)
    if lines:
        output.append(
            (_key(date_path, session_id, len(output)), b''.join(lines))
        )
    return output


def _line(record: dict[str, object]) -> bytes:
    return (
        json.dumps(record, sort_keys=True, separators=(',', ':')).encode()
        + b'\n'
    )


def _date_path(value: object) -> str:
    return (
        str(value or '1970-01-01T00')[:13].replace('-', '/').replace('T', '/')
    )


def _key(date_path: str, session_id: str, index: int) -> str:
    return f'training/v2/{date_path}/vm_session_{session_id}_{index:04}.jsonl'
