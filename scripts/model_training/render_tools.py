"""Recover tool schemas so rendered text matches the inference prompt.

Chat templates inject an available-tools block into the system prompt at
inference time. Transcripts that omit it teach the model to echo the tool
list instead of calling a tool, so any conversation containing tool calls is
rendered with the schemas of the tools it actually used.
"""

import json


def _arguments(call: dict[str, object]) -> dict[str, object]:
    function = call.get('function')
    if not isinstance(function, dict):
        return {}
    try:
        parsed = json.loads(str(function.get('arguments') or '{}'))
    except json.JSONDecodeError:
        return {}
    return parsed if isinstance(parsed, dict) else {}


def collect(messages: list[dict[str, object]]) -> list[dict[str, object]]:
    """Return tool specs inferred from the tool calls in a conversation."""
    observed: dict[str, set[str]] = {}
    for message in messages:
        for call in message.get('tool_calls') or []:
            if not isinstance(call, dict):
                continue
            function = call.get('function')
            if not isinstance(function, dict):
                continue
            name = str(function.get('name') or '').strip()
            if not name:
                continue
            keys = observed.setdefault(name, set())
            keys.update(str(k) for k in _arguments(call))
    return [
        _spec(name, sorted(keys)) for name, keys in sorted(observed.items())
    ]


def _spec(name: str, keys: list[str]) -> dict[str, object]:
    return {
        'type': 'function',
        'function': {
            'name': name,
            'description': f'CodeTether {name} tool.',
            'parameters': {
                'type': 'object',
                'properties': {k: {'type': 'string'} for k in keys},
            },
        },
    }
