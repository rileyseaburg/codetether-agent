"""Safe accessors for loosely sourced training records."""

from typing import cast

from .model import JsonObject, SourceRecord


def metadata(record: SourceRecord) -> JsonObject:
    """Return record metadata or an empty mapping."""
    value = record.value.get('metadata')
    return cast(JsonObject, value) if isinstance(value, dict) else {}


def text(record: SourceRecord, key: str) -> str | None:
    """Read a string field from a record."""
    value = record.value.get(key)
    return value if isinstance(value, str) else None


def meta_text(record: SourceRecord, key: str) -> str | None:
    """Read a string field from record metadata."""
    value = metadata(record).get(key)
    return value if isinstance(value, str) else None


def timestamp(record: SourceRecord) -> tuple[str, int]:
    """Return a deterministic sortable timestamp key."""
    return meta_text(record, 'timestamp') or '', record.line


def message(record: SourceRecord) -> JsonObject:
    """Strip provenance fields to produce a trainer-facing message."""
    keys = ('role', 'content', 'tool_calls', 'tool_call_id', 'name')
    return {key: record.value[key] for key in keys if key in record.value}


def is_user(record: SourceRecord) -> bool:
    """Return whether this record contains user input."""
    return text(record, 'role') == 'user'
