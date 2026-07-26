"""Convert CodeTether session content parts into chat messages."""

from .session_tool import tool_message
from .tool_arguments import mapping


def convert(message: dict[str, object]) -> dict[str, object] | None:
    """Return a chat message for one stored session message."""
    role = str(message.get('role', ''))
    parts = message.get('content')
    if role not in {'user', 'assistant', 'tool'} or not isinstance(parts, list):
        return None
    if role == 'tool':
        return tool_message(parts)
    text = _text(parts)
    calls = _calls(parts)
    if role == 'assistant' and calls:
        return {'role': 'assistant', 'content': text, 'tool_calls': calls}
    return {'role': role, 'content': text} if text else None


def _text(parts: list[object]) -> str:
    """Join visible text, excluding private reasoning."""
    return ''.join(
        str(p.get('text', ''))
        for p in parts
        if isinstance(p, dict) and p.get('type') == 'text'
    ).strip()


def _calls(parts: list[object]) -> list[dict[str, object]]:
    result: list[dict[str, object]] = []
    for part in parts:
        if not isinstance(part, dict) or part.get('type') != 'tool_call':
            continue
        arguments = part.get('arguments') or part.get('input') or {}
        result.append(
            {
                'id': str(part.get('id') or part.get('tool_call_id') or ''),
                'type': 'function',
                'function': {
                    'name': str(part.get('name', '')),
                    'arguments': mapping(arguments),
                },
            }
        )
    return result
