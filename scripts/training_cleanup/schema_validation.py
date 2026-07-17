"""Trainer-message shape and timestamp validation."""

from .model import SourceRecord
from .record import meta_text, text
from .timestamp_validation import valid as valid_timestamp
from .tool_schema import valid_calls


_ROLES = {'system', 'user', 'assistant', 'tool'}


def rejection_reason(record: SourceRecord) -> str | None:
    """Return a schema rejection reason for malformed messages."""
    role = text(record, 'role')
    if role not in _ROLES:
        return 'invalid_role'
    if not valid_timestamp(meta_text(record, 'timestamp')):
        return 'invalid_timestamp'
    content = record.value.get('content')
    if content is not None and not isinstance(content, str):
        return 'invalid_content'
    if role in {'system', 'user', 'tool'} and not isinstance(content, str):
        return 'missing_content'
    if role == 'tool' and not text(record, 'tool_call_id'):
        return 'missing_tool_call_id'
    if (
        role == 'assistant'
        and content is None
        and not valid_calls(record.value.get('tool_calls'))
    ):
        return 'empty_assistant'
    return None
