"""Typed quarantine rows with sensitive payload minimization."""

import json

from .model import JsonObject, RejectedRecord
from .provenance import identity
from .row_common import digest


_REDACTED = {'embedded_image', 'oversized_content', 'possible_secret'}


def build(
    record: RejectedRecord,
    source_uri: str,
    source_sha256: str,
    run_id: str,
    cleaned_at: str,
) -> JsonObject:
    """Convert one rejection into an auditable typed row."""
    resolved_uri = record.source_uri or source_uri
    resolved_sha256 = record.source_sha256 or source_sha256
    row_identity = f'v2:{identity(record)}:{record.reason}'
    raw = (
        None
        if record.reason in _REDACTED
        else json.dumps(record.value, sort_keys=True)
    )
    return {
        'run_id': run_id,
        'quarantine_id': digest(row_identity),
        'cleanup_version': 2,
        'source_uri': resolved_uri,
        'source_sha256': resolved_sha256,
        'source_line': record.line,
        'reason': record.reason,
        'record_json': raw,
        'cleaned_at': cleaned_at,
    }
