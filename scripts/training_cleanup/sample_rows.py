"""Typed accepted-sample rows for the Iceberg sink."""

from typing import cast

from .model import JsonObject
from .row_common import digest


def build(
    sample: JsonObject, run_id: str, source_sha256: str, cleaned_at: str
) -> JsonObject:
    """Flatten one cleaned conversation into the samples table schema."""
    metadata = cast(JsonObject, sample['metadata'])
    identity = (
        f'v{metadata["cleanup_version"]}:{source_sha256}:{metadata["correlation_id"]}:'
        f'{metadata["source_lines"]}'
    )
    return {
        'run_id': run_id,
        'sample_id': digest(identity),
        'cleanup_version': metadata['cleanup_version'],
        'source_uri': metadata['source_uri'],
        'source_sha256': source_sha256,
        'source_lines': metadata['source_lines'],
        'sender_id': metadata['sender_id'],
        'correlation_id': metadata['correlation_id'],
        'quality_tier': metadata['quality_tier'],
        'start_timestamp': metadata['start_timestamp'],
        'end_timestamp': metadata['end_timestamp'],
        'messages': sample['messages'],
        'cleaned_at': cleaned_at,
    }
