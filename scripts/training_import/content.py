"""Flatten native message parts without losing tool structure."""

import json


def text(parts: list[object]) -> str | None:
    """Join text and losslessly serialize unsupported content parts."""
    values: list[str] = []
    for part in parts:
        if not isinstance(part, dict):
            values.append(str(part))
        elif part.get('type') == 'text' and isinstance(part.get('text'), str):
            values.append(part['text'])
        elif part.get('type') not in {'tool_call', 'tool_result'}:
            values.append(
                json.dumps(part, sort_keys=True, separators=(',', ':'))
            )
    joined = '\n'.join(value for value in values if value)
    return joined or None


def calls(parts: list[object]) -> list[dict[str, object]]:
    """Convert native tool-call parts into chat-completion calls."""
    return [
        {
            'id': str(part['id']),
            'type': 'function',
            'function': {
                'name': str(part.get('name') or 'unknown'),
                'arguments': _arguments(part.get('arguments')),
            },
        }
        for part in parts
        if isinstance(part, dict)
        and part.get('type') == 'tool_call'
        and part.get('id')
    ]


def _arguments(value: object) -> str:
    return (
        value if isinstance(value, str) else json.dumps(value, sort_keys=True)
    )
