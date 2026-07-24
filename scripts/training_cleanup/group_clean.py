"""Cross-object cleaning for one sender/correlation group."""

from .correlation import GroupKey
from .group_dedup import records as deduplicate
from .group_result import GroupResult
from .group_turn import append
from .model import SourceRecord
from .record import source_order
from .sample_quality import reject
from .turns import split as split_turns


def clean(
    item: tuple[GroupKey, list[SourceRecord]],
    max_messages: int,
    max_characters: int,
) -> GroupResult:
    """Reconstruct, validate, and chunk one complete correlation group."""
    (sender, correlation), values = item
    records, duplicate_rejections = deduplicate(
        sorted(values, key=source_order)
    )
    prefix, turns = split_turns(records)
    result = GroupResult(rejected=duplicate_rejections)
    result.rejected.extend(reject(prefix, 'missing_user_prompt'))
    for turn in turns:
        append(
            result,
            turn,
            (sender, correlation),
            (max_messages, max_characters),
        )
    return result
