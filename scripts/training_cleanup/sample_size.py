"""Trainer-sample size accounting."""

from .model import SourceRecord
from .record import message
from .serialization import json_line


def characters(records: list[SourceRecord]) -> int:
    """Count serialized trainer-message characters."""
    return sum(len(json_line(message(record))) for record in records)


def fits(
    records: list[SourceRecord], max_messages: int, max_characters: int
) -> bool:
    """Return whether records fit both configured sample limits."""
    return (
        len(records) <= max_messages and characters(records) <= max_characters
    )
