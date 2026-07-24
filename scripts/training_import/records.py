"""Map native session messages into training-v2 source records."""

from .content import calls, text
from .record_metadata import build as metadata
from .tool_record import build as tool


def session(value: dict[str, object]) -> list[dict[str, object]]:
    """Convert every supported message while preserving original order."""
    session_id = str(value.get('id') or 'unknown')
    created = value.get('created_at')
    names: dict[str, str] = {}
    output: list[dict[str, object]] = []
    for index, message in enumerate(value.get('messages') or []):
        if not isinstance(message, dict):
            continue
        parts = (
            message.get('content')
            if isinstance(message.get('content'), list)
            else []
        )
        for call in calls(parts):
            function = call['function']
            if isinstance(function, dict):
                names[str(call['id'])] = str(function['name'])
        meta = metadata(session_id, created, index)
        output.extend(_message(message, parts, names, meta))
    return output


def _message(message, parts, names, metadata) -> list[dict[str, object]]:
    role = (
        'system'
        if message.get('role') == 'developer'
        else str(message.get('role'))
    )
    if role == 'tool':
        return [
            tool(part, names, metadata)
            for part in parts
            if isinstance(part, dict) and part.get('type') == 'tool_result'
        ]
    record: dict[str, object] = {'role': role, 'metadata': metadata}
    if (content := text(parts)) is not None:
        record['content'] = content
    if tool_calls := calls(parts):
        record['tool_calls'] = tool_calls
    return [record]
