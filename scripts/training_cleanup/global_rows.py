"""Typed tagged rows from global cleanup values."""

from .model import JsonObject, RejectedRecord
from .quarantine_rows import build as quarantine_row
from .sample_rows import build as sample_row
from .serialization import json_line
from .spark_transform import TaggedLine


def sample(value: JsonObject, run_id: str, cleaned_at: str) -> TaggedLine:
    """Serialize one trainer sample row."""
    return 'samples', json_line(sample_row(value, run_id, '', cleaned_at))


def quarantine(
    record: RejectedRecord, run_id: str, cleaned_at: str
) -> TaggedLine:
    """Serialize one provenance-bearing quarantine row."""
    row = quarantine_row(record, '', '', run_id, cleaned_at)
    return 'quarantine', json_line(row)


def manifest(value: dict[str, object]) -> TaggedLine:
    """Serialize one source-object manifest row."""
    return 'manifests', json_line(value)
