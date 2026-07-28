"""Render assistant tool calls in the model's native syntax.

Ingestion recorded 9,943 assistant messages carrying `tool_calls`, but the
v4 render emitted only their prose. Sampling 6,000 rendered pairs found tool
calls in 4,989 prompts and 0 completions, so loss never covered a single
emitted call and the corpus could not teach our tool schema.

Qwen3.5 nests `<function=name>` inside `<tool_call>`, with one
`<parameter=name>` block per argument.
"""

import json


def render_calls(calls: list[dict[str, object]]) -> str:
    """Return native-syntax tool calls for one assistant turn."""
    return ''.join(_one(call) for call in calls if _name_of(call))


def _one(call: dict[str, object]) -> str:
    """Return a single rendered call."""
    name = _name_of(call)
    body = ''.join(
        f'<parameter={key}>\n{_stringify(value)}\n</parameter>\n'
        for key, value in _arguments_of(call).items()
    )
    return f'<tool_call>\n<function={name}>\n{body}</function>\n</tool_call>\n'


def _name_of(call: dict[str, object]) -> str:
    """Return the called function name, empty when malformed."""
    function = call.get('function')
    if not isinstance(function, dict):
        return ''
    return str(function.get('name') or '')


def _arguments_of(call: dict[str, object]) -> dict[str, object]:
    """Return call arguments, tolerating JSON-encoded strings."""
    function = call.get('function')
    raw = function.get('arguments') if isinstance(function, dict) else None
    if isinstance(raw, str):
        try:
            raw = json.loads(raw)
        except json.JSONDecodeError:
            return {}
    return raw if isinstance(raw, dict) else {}


def _stringify(value: object) -> str:
    """Return a parameter value as text."""
    if isinstance(value, str):
        return value
    return json.dumps(value, sort_keys=True)
