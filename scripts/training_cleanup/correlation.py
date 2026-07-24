"""Correlation keys and uncorrelated-record handling."""

from collections.abc import Iterator

from .model import RejectedRecord, SourceRecord
from .prepared_model import PreparedObject
from .provenance import reject
from .record import is_user, meta_text


GroupKey = tuple[str, str]
KeyedRecord = tuple[GroupKey, SourceRecord]


def pairs(prepared: PreparedObject) -> Iterator[KeyedRecord]:
    """Yield records keyed for cross-object reconstruction."""
    for record in prepared.records:
        correlation = meta_text(record, 'correlation_id')
        if correlation:
            yield (
                (
                    meta_text(record, 'sender_id') or 'unknown',
                    correlation,
                ),
                record,
            )


def uncorrelated(prepared: PreparedObject) -> Iterator[RejectedRecord]:
    """Quarantine records that cannot be joined to a conversation."""
    for record in prepared.records:
        if not meta_text(record, 'correlation_id'):
            reason = (
                'orphan_user_prompt'
                if is_user(record)
                else 'missing_correlation'
            )
            yield reject(record, reason)
