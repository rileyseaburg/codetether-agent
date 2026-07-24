"""Per-object accounting events from global cleanup outcomes."""

from collections.abc import Iterator
from typing import cast

from .model import JsonObject, RejectedRecord
from .provenance import identity
from .sample_rows import build as sample_row


Outcome = tuple[str, str, str]
KeyedOutcome = tuple[str, Outcome]


def sample(
    value: JsonObject, run_id: str, cleaned_at: str
) -> Iterator[KeyedOutcome]:
    """Mark every raw origin accepted and attribute one emitted sample."""
    row = sample_row(value, run_id, '', cleaned_at)
    metadata = cast(JsonObject, value['metadata'])
    origins = cast(list[JsonObject], metadata['source_records'])
    seen: set[str] = set()
    for origin in origins:
        uri = cast(str, origin['uri'])
        key = f'{uri}:{origin["sha256"]}:{origin["line"]}'
        if key not in seen:
            seen.add(key)
            yield uri, ('accepted', key, '')
    yield cast(str, metadata['source_uri']), ('sample', row['sample_id'], '')


def rejection(record: RejectedRecord) -> KeyedOutcome:
    """Mark one raw record rejected for its source-object manifest."""
    return record.source_uri, ('rejected', identity(record), record.reason)
