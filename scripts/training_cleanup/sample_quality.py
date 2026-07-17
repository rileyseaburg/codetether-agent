"""Conversation quality classification and bulk rejection."""

from .model import RejectedRecord, SourceRecord
from .record import meta_text, text


def quality(records: list[SourceRecord]) -> str:
    """Return the accepted sample's trainer-facing quality tier."""
    return (
        'complete'
        if any(_visible_assistant(record) for record in records)
        else 'tool_use'
    )


def reject(records: list[SourceRecord], reason: str) -> list[RejectedRecord]:
    """Reject all records in an unusable correlation group."""
    return [
        RejectedRecord(record.line, reason, record.value) for record in records
    ]


def _visible_assistant(record: SourceRecord) -> bool:
    return (
        text(record, 'role') == 'assistant'
        and bool(text(record, 'content'))
        and meta_text(record, 'bus_kind') == 'agent_message'
    )
