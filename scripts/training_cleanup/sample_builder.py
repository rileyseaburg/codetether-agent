"""Trainer sample construction with cross-object provenance."""

from .model import JsonObject, SourceRecord
from .provenance import value as provenance
from .record import message, timestamp
from .sample_quality import quality
from .sample_size import characters


def build(
    records: list[SourceRecord],
    sender: str,
    correlation: str,
    chunk_index: int,
    chunk_count: int,
) -> JsonObject:
    """Build one bounded sample from provenance-bearing records."""
    origins = [provenance(record) for record in records]
    return {
        'messages': [message(record) for record in records],
        'metadata': {
            'cleanup_version': 2,
            'source_uri': records[0].source_uri,
            'source_lines': [record.line for record in records],
            'source_records': origins,
            'sender_id': sender,
            'correlation_id': correlation,
            'quality_tier': quality(records),
            'chunk_index': chunk_index,
            'chunk_count': chunk_count,
            'message_chars': characters(records),
            'start_timestamp': timestamp(records[0])[0],
            'end_timestamp': timestamp(records[-1])[0],
        },
    }
