"""Pure orchestration for cleaning one source JSONL object."""

from .filter import records as filter_records
from .group import conversations
from .model import CleanResult
from .parse import jsonl
from .record import timestamp
from .tool_pairs import records as pair_tools


def object_text(
    source_uri: str, text: str, max_content_chars: int
) -> CleanResult:
    """Clean one immutable source object into samples and quarantine entries."""
    parsed, rejected, input_lines = jsonl(text)
    filtered, filter_rejections = filter_records(parsed, max_content_chars)
    paired, pair_rejections = pair_tools(filtered)
    samples, group_rejections = conversations(
        sorted(paired, key=timestamp), source_uri
    )
    return CleanResult(
        samples=samples,
        rejected=rejected
        + filter_rejections
        + pair_rejections
        + group_rejections,
        input_lines=input_lines,
    )
