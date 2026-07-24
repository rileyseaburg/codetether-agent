"""Pending uncorrelated user-prompt replacement."""

from .model import RejectedRecord, SourceRecord
from .provenance import reject


def replace(
    pending: dict[str, SourceRecord],
    sender: str,
    record: SourceRecord,
    rejected: list[RejectedRecord],
) -> None:
    """Store the newest prompt and quarantine an older orphan."""
    previous = pending.get(sender)
    if previous:
        rejected.append(reject(previous, 'orphan_user_prompt'))
    pending[sender] = record
