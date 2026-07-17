"""Record-level noise, duplicate, and safety filtering."""

import json

from .model import RejectedRecord, SourceRecord
from .record import meta_text
from .schema_validation import rejection_reason as schema_rejection
from .security import rejection_reason


_NOISE = {'heartbeat': 'heartbeat', 'tool_output_full': 'duplicate_tool_output'}


def records(
    source: list[SourceRecord], max_content_chars: int
) -> tuple[list[SourceRecord], list[RejectedRecord]]:
    """Partition raw records into candidates and auditable rejections."""
    accepted: list[SourceRecord] = []
    rejected: list[RejectedRecord] = []
    seen: set[str] = set()
    for record in source:
        canonical = json.dumps(
            record.value, sort_keys=True, separators=(',', ':')
        )
        reason = _NOISE.get(meta_text(record, 'bus_kind') or '')
        if canonical in seen:
            reason = 'exact_duplicate'
        seen.add(canonical)
        reason = reason or schema_rejection(record)
        reason = reason or rejection_reason(record, max_content_chars)
        if reason:
            rejected.append(RejectedRecord(record.line, reason, record.value))
        else:
            accepted.append(record)
    return accepted, rejected
