"""Convert Codex rollout payloads into chat message dictionaries."""

import json

from .tool_arguments import mapping


MAX_OUTPUT = 4000


def user_text(payload: dict[str, object]) -> dict[str, object] | None:
    """Return a user message, skipping injected instruction blocks."""
    text = _text(payload, 'input_text')
    if not text or text.startswith(('# AGENTS.md', '<permissions')):
        return None
    return {'role': 'user', 'content': text}


def assistant_text(payload: dict[str, object]) -> dict[str, object] | None:
    """Return an assistant message when it carries visible output."""
    text = _text(payload, 'output_text')
    return {'role': 'assistant', 'content': text} if text else None


def tool_call(payload: dict[str, object]) -> dict[str, object]:
    """Return an assistant turn carrying a native tool call."""
    return {
        'role': 'assistant',
        'content': '',
        'tool_calls': [
            {
                'id': str(payload.get('call_id', '')),
                'type': 'function',
                'function': {
                    'name': str(payload.get('name', '')),
                    'arguments': mapping(payload.get('arguments', '{}')),
                },
            }
        ],
    }


def tool_output(payload: dict[str, object]) -> dict[str, object]:
    """Return a bounded tool result message."""
    raw = payload.get('output')
    text = raw if isinstance(raw, str) else json.dumps(raw)
    return {
        'role': 'tool',
        'tool_call_id': str(payload.get('call_id', '')),
        'content': text[:MAX_OUTPUT],
    }


def _text(payload: dict[str, object], kind: str) -> str:
    content = payload.get('content')
    if not isinstance(content, list):
        return ''
    parts = [
        str(p.get('text', ''))
        for p in content
        if isinstance(p, dict) and p.get('type') == kind
    ]
    return ''.join(parts).strip()
