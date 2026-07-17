"""Owners of tool-call and tool-response identifiers."""

from .model import SourceRecord
from .record import text
from .tool_ids import call_ids


def calls(source: list[SourceRecord]) -> dict[str, list[SourceRecord]]:
    """Index assistant tool calls by identifier."""
    owners: dict[str, list[SourceRecord]] = {}
    for record in source:
        for identifier in call_ids(record):
            owners.setdefault(identifier, []).append(record)
    return owners


def responses(source: list[SourceRecord]) -> dict[str, list[SourceRecord]]:
    """Index tool-role responses by identifier."""
    owners: dict[str, list[SourceRecord]] = {}
    for record in source:
        identifier = text(record, 'tool_call_id')
        if identifier:
            owners.setdefault(identifier, []).append(record)
    return owners
