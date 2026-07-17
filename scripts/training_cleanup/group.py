"""Conversation reconstruction from chronological bus records."""

from collections import defaultdict
from typing import TypeAlias

from .group_samples import build
from .model import JsonObject, RejectedRecord, SourceRecord
from .pending_user import replace
from .record import is_user, meta_text, text, timestamp


GroupKey: TypeAlias = tuple[str, str]


def conversations(
    source: list[SourceRecord], source_uri: str
) -> tuple[list[JsonObject], list[RejectedRecord]]:
    """Build correlated chat samples and quarantine orphaned records."""
    groups: dict[GroupKey, list[SourceRecord]] = defaultdict(list)
    pending_users: dict[str, SourceRecord] = {}
    rejected: list[RejectedRecord] = []
    for record in sorted(source, key=timestamp):
        sender = meta_text(record, 'sender_id') or 'unknown'
        correlation = meta_text(record, 'correlation_id')
        if is_user(record) and not correlation:
            replace(pending_users, sender, record, rejected)
        elif correlation:
            key = sender, correlation
            if key not in groups and sender in pending_users:
                groups[key].append(pending_users.pop(sender))
            groups[key].append(record)
        elif text(record, 'role') == 'assistant' and sender in pending_users:
            key = sender, f'legacy-line-{pending_users[sender].line}'
            groups[key].extend((pending_users.pop(sender), record))
        else:
            rejected.append(
                RejectedRecord(record.line, 'missing_correlation', record.value)
            )
    rejected.extend(
        RejectedRecord(record.line, 'orphan_user_prompt', record.value)
        for record in pending_users.values()
    )
    samples = build(groups, source_uri, rejected)
    return samples, rejected
