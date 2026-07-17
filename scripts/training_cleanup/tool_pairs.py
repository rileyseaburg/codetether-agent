"""Validation of assistant tool calls and tool-role responses."""

from .model import RejectedRecord, SourceRecord
from .tool_owners import calls as call_owners
from .tool_owners import responses as response_owners


def records(
    source: list[SourceRecord],
) -> tuple[list[SourceRecord], list[RejectedRecord]]:
    """Reject missing, duplicate, or mismatched tool-call relationships."""
    calls = call_owners(source)
    responses = response_owners(source)
    shared = calls.keys() & responses.keys()
    valid = {
        key for key in shared if len(calls[key]) == len(responses[key]) == 1
    }
    rejected_lines: set[int] = set()
    for key, owners in calls.items():
        if key not in valid:
            rejected_lines.update(record.line for record in owners)
    for key, owners in responses.items():
        if key not in valid:
            rejected_lines.update(record.line for record in owners)
    accepted = [
        record for record in source if record.line not in rejected_lines
    ]
    rejected = [
        RejectedRecord(record.line, 'unpaired_or_duplicate_tool', record.value)
        for record in source
        if record.line in rejected_lines
    ]
    return accepted, rejected
