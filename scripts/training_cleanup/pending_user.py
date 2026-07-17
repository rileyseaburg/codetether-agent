"""Pending uncorrelated user-prompt replacement."""

from .model import RejectedRecord, SourceRecord


def replace(
    pending: dict[str, SourceRecord],
    sender: str,
    record: SourceRecord,
    rejected: list[RejectedRecord],
) -> None:
    """Store the newest prompt and quarantine an older orphan."""
    previous = pending.get(sender)
    if previous:
        rejected.append(
            RejectedRecord(previous.line, 'orphan_user_prompt', previous.value)
        )
    pending[sender] = record
