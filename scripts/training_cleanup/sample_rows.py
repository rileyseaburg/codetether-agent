"""Typed accepted-sample rows for the Iceberg sink."""

from typing import cast

from .model import JsonObject
from .row_common import digest
from .serialization import json_line


def build(
    sample: JsonObject, run_id: str, source_sha256: str, cleaned_at: str
) -> JsonObject:
    """Flatten one cleaned conversation into the samples table schema."""
    metadata = cast(JsonObject, sample['metadata'])
    origins = metadata.get('source_records')
    origins = origins if isinstance(origins, list) else []
    combined_sha256 = digest(json_line(origins)) if origins else source_sha256
    identity = (
        f'v{metadata["cleanup_version"]}:{metadata["correlation_id"]}:'
        f'{metadata.get("chunk_index", 0)}:{combined_sha256}'
    )
    return {
        'run_id': run_id,
        'sample_id': digest(identity),
        'cleanup_version': metadata['cleanup_version'],
        'source_uri': metadata['source_uri'],
        'source_sha256': combined_sha256,
        'source_lines': metadata['source_lines'],
        'source_records': origins or None,
        'sender_id': metadata['sender_id'],
        'correlation_id': metadata['correlation_id'],
        'quality_tier': metadata['quality_tier'],
        'chunk_index': metadata.get('chunk_index', 0),
        'chunk_count': metadata.get('chunk_count', 1),
        'message_chars': metadata.get('message_chars'),
        'start_timestamp': metadata['start_timestamp'],
        'end_timestamp': metadata['end_timestamp'],
        'messages': sample['messages'],
        'cleaned_at': cleaned_at,
    }
