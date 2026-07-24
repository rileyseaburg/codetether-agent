"""Cross-object duplicate record removal."""

import json

from .model import RejectedRecord, SourceRecord
from .provenance import reject


def records(
    source: list[SourceRecord],
) -> tuple[list[SourceRecord], list[RejectedRecord]]:
    """Retain the first canonical record and quarantine later copies."""
    accepted: list[SourceRecord] = []
    rejected: list[RejectedRecord] = []
    seen: set[str] = set()
    for record in source:
        canonical = json.dumps(
            record.value, sort_keys=True, separators=(',', ':')
        )
        if canonical in seen:
            rejected.append(reject(record, 'exact_duplicate'))
        else:
            seen.add(canonical)
            accepted.append(record)
    return accepted, rejected
