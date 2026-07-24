"""Stable raw-record provenance helpers."""

from .model import JsonObject, RejectedRecord, SourceRecord


def identity(record: SourceRecord | RejectedRecord) -> str:
    """Return an object-and-line identity for one immutable raw record."""
    return f'{record.source_uri}:{record.source_sha256}:{record.line}'


def reject(record: SourceRecord, reason: str) -> RejectedRecord:
    """Create a rejection without losing its raw source location."""
    return RejectedRecord(
        record.line,
        reason,
        record.value,
        record.source_uri,
        record.source_sha256,
    )


def value(record: SourceRecord) -> JsonObject:
    """Return trainer-facing provenance for one source record."""
    return {
        'uri': record.source_uri,
        'sha256': record.source_sha256,
        'line': record.line,
    }
