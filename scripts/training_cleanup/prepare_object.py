"""Record-level preparation for cross-object reconstruction."""

from .filter import records as filter_records
from .parse import jsonl
from .prepared_model import PreparedObject
from .serialization import digest


def build(item: tuple[str, str], max_content_chars: int) -> PreparedObject:
    """Parse and validate one immutable object while retaining provenance."""
    source_uri, source_text = item
    source_sha256 = digest(source_text)
    parsed, rejected, input_lines = jsonl(
        source_text, source_uri, source_sha256
    )
    records, filtered = filter_records(parsed, max_content_chars)
    return PreparedObject(
        source_uri,
        source_sha256,
        input_lines,
        records,
        rejected + filtered,
    )
