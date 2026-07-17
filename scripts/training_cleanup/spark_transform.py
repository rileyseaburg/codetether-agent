"""Spark-worker transformation of one immutable source object."""

from collections.abc import Iterator
from typing import TypeAlias, cast

from .clean import object_text
from .format import manifest
from .manifest_rows import build as manifest_row
from .quarantine_rows import build as quarantine_row
from .row_common import cleaned_at
from .sample_rows import build as sample_row
from .serialization import json_line


TaggedLine: TypeAlias = tuple[str, str]


def object_lines(
    item: tuple[str, str], max_content_chars: int, run_id: str
) -> Iterator[TaggedLine]:
    """Yield accepted, quarantined, and manifest lines for one object."""
    source_uri, source_text = item
    result = object_text(source_uri, source_text, max_content_chars)
    source_sha256 = cast(
        str, manifest(result, source_uri, source_text)['source_sha256']
    )
    timestamp = cleaned_at()
    for sample in result.samples:
        yield (
            'samples',
            json_line(sample_row(sample, run_id, source_sha256, timestamp)),
        )
    for record in result.rejected:
        row = quarantine_row(
            record, source_uri, source_sha256, run_id, timestamp
        )
        yield 'quarantine', json_line(row)
    row = manifest_row(result, source_uri, source_text, run_id, timestamp)
    yield 'manifests', json_line(row)
