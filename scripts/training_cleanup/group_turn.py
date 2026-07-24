"""Validation and chunking for one correlated user turn."""

from .chunk_turn import split as chunks
from .correlation import GroupKey
from .group_result import GroupResult
from .model import SourceRecord
from .record import text
from .sample_builder import build
from .sample_quality import reject
from .tool_pairs import records as pair_tools


def append(
    result: GroupResult,
    turn: list[SourceRecord],
    key: GroupKey,
    limits: tuple[int, int],
) -> None:
    """Append bounded samples or auditable rejections for one turn."""
    sender, correlation = key
    max_messages, max_characters = limits
    paired, pair_rejections = pair_tools(turn)
    result.rejected.extend(pair_rejections)
    if not any(text(record, 'role') == 'assistant' for record in paired):
        result.rejected.extend(reject(paired, 'missing_assistant_target'))
        return
    values, chunk_rejections = chunks(paired, max_messages, max_characters)
    result.rejected.extend(chunk_rejections)
    result.samples.extend(
        build(records, sender, correlation, index, len(values))
        for index, records in enumerate(values)
    )
