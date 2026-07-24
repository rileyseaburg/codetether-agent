"""Tool-safe chunking for one validated user turn."""

from .model import RejectedRecord, SourceRecord
from .provenance import reject
from .record import text
from .sample_size import fits
from .tool_units import build as units

Records = list[SourceRecord]
ChunkResult = tuple[list[Records], list[RejectedRecord]]


def split(turn: Records, max_messages: int, max_characters: int) -> ChunkResult:
    """Chunk a turn without separating assistant calls from tool results."""
    prompt = next(
        index for index, row in enumerate(turn) if text(row, 'role') == 'user'
    )
    chunks = []
    rejected: list[RejectedRecord] = []
    current = (context := turn[: prompt + 1]).copy()
    for unit in units(turn[prompt + 1 :]):
        candidate = current + unit
        if fits(candidate, max_messages, max_characters):
            current = candidate
            continue
        if len(current) > len(context):
            chunks.append(current)
            current = context.copy()
        if fits(current + unit, max_messages, max_characters):
            current.extend(unit)
        else:
            rejected.extend(
                reject(row, 'oversized_sample_unit') for row in unit
            )
    if len(current) > len(context):
        chunks.append(current)
    accepted, rejected = _with_targets(chunks, rejected)
    if not accepted:
        rejected.append(reject(context[-1], 'no_trainable_chunk'))
    return accepted, rejected


def _with_targets(
    chunks: list[list[SourceRecord]], rejected: list[RejectedRecord]
) -> tuple[list[list[SourceRecord]], list[RejectedRecord]]:
    accepted = [
        chunk
        for chunk in chunks
        if any(text(record, 'role') == 'assistant' for record in chunk)
    ]
    for chunk in chunks:
        if chunk not in accepted:
            rejected.extend(
                reject(row, 'missing_assistant_target') for row in chunk[1:]
            )
    return accepted, rejected
