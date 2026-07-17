"""Tool-call identifier extraction from source records."""

from typing import cast

from .model import JsonObject, SourceRecord


def call_ids(record: SourceRecord) -> list[str]:
    """Return every well-shaped assistant tool-call ID."""
    value = record.value.get('tool_calls')
    if not isinstance(value, list):
        return []
    calls = cast(list[object], value)
    return [identifier for item in calls if (identifier := _identifier(item))]


def _identifier(value: object) -> str | None:
    if not isinstance(value, dict):
        return None
    identifier = cast(JsonObject, value).get('id')
    return identifier if isinstance(identifier, str) else None
