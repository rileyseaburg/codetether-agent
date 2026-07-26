"""Render stored tool results as bounded tool messages."""

import json


MAX_OUTPUT = 4000


def tool_message(parts: list[object]) -> dict[str, object] | None:
    """Return a tool message from stored `tool_result` parts."""
    for part in parts:
        if not isinstance(part, dict) or part.get('type') != 'tool_result':
            continue
        content = part.get('content')
        text = content if isinstance(content, str) else json.dumps(content)
        return {
            'role': 'tool',
            'tool_call_id': str(
                part.get('tool_call_id') or part.get('id') or ''
            ),
            'content': text[:MAX_OUTPUT],
        }
    return None
