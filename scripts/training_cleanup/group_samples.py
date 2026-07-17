"""Build trainer-facing samples from validated correlation groups."""

from typing import TypeAlias

from .model import JsonObject, RejectedRecord, SourceRecord
from .record import is_user, message, text, timestamp
from .sample_quality import quality, reject


GroupKey: TypeAlias = tuple[str, str]


def build(
    groups: dict[GroupKey, list[SourceRecord]],
    source_uri: str,
    rejected: list[RejectedRecord],
) -> list[JsonObject]:
    """Convert valid groups and quarantine groups without training targets."""
    output: list[JsonObject] = []
    for (sender, correlation), records in groups.items():
        records.sort(key=timestamp)
        if not any(is_user(record) for record in records):
            rejected.extend(reject(records, 'missing_user_prompt'))
        elif not any(text(record, 'role') == 'assistant' for record in records):
            rejected.extend(reject(records, 'missing_assistant_target'))
        else:
            output.append(_sample(records, source_uri, sender, correlation))
    return output


def _sample(
    records: list[SourceRecord], source_uri: str, sender: str, correlation: str
) -> JsonObject:
    return {
        'messages': [message(record) for record in records],
        'metadata': {
            'cleanup_version': 1,
            'source_uri': source_uri,
            'source_lines': [record.line for record in records],
            'sender_id': sender,
            'correlation_id': correlation,
            'quality_tier': quality(records),
            'start_timestamp': timestamp(records[0])[0],
            'end_timestamp': timestamp(records[-1])[0],
        },
    }
