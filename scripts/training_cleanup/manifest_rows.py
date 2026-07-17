"""Typed per-source manifests for the Iceberg audit table."""

from .format import manifest
from .model import CleanResult, JsonObject


def build(
    result: CleanResult,
    source_uri: str,
    source_text: str,
    run_id: str,
    cleaned_at: str,
) -> JsonObject:
    """Add run identity and cleanup time to a stable source manifest."""
    value = manifest(result, source_uri, source_text)
    return {'run_id': run_id, **value, 'cleaned_at': cleaned_at}
