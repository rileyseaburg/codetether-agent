"""Load one CodeTether session file as ordered chat messages."""

import json

from pathlib import Path

from .session_parts import convert


def messages(path: Path) -> list[dict[str, object]]:
    """Return the conversation stored in a session JSON file."""
    record = _load(path)
    if record is None:
        return []
    stored = record.get('messages')
    if not isinstance(stored, list):
        return []
    result: list[dict[str, object]] = []
    for message in stored:
        if not isinstance(message, dict):
            continue
        converted = convert(message)
        if converted is not None:
            result.append(converted)
    return result


def _load(path: Path) -> dict[str, object] | None:
    try:
        value = json.loads(path.read_text(errors='ignore'))
    except (ValueError, OSError):
        return None
    return value if isinstance(value, dict) else None
