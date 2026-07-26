"""Parse Codex rollout transcripts into ordered chat messages.

Rollout files hold the richest available supervision: real user requests,
assistant replies, and `function_call` / `function_call_output` pairs. The
governed Iceberg table lacked native tool calls, which is why the earlier
fine-tune never learned to emit them.
"""

import json

from pathlib import Path

from .codex_parts import assistant_text, tool_call, tool_output, user_text


KEEP = {'message', 'function_call', 'function_call_output'}


def messages(path: str) -> list[dict[str, object]]:
    """Return one conversation as ordered chat messages."""
    result: list[dict[str, object]] = []
    with Path(path).open(errors='ignore') as handle:
        for line in handle:
            payload = _payload(line)
            if payload is None:
                continue
            converted = _convert(payload)
            if converted is not None:
                result.append(converted)
    return result


def _payload(line: str) -> dict[str, object] | None:
    try:
        record = json.loads(line)
    except ValueError:
        return None
    if not isinstance(record, dict) or record.get('type') != 'response_item':
        return None
    payload = record.get('payload')
    if not isinstance(payload, dict) or payload.get('type') not in KEEP:
        return None
    return payload


def _convert(payload: dict[str, object]) -> dict[str, object] | None:
    kind = payload.get('type')
    if kind == 'function_call':
        return tool_call(payload)
    if kind == 'function_call_output':
        return tool_output(payload)
    role = str(payload.get('role', ''))
    if role == 'user':
        return user_text(payload)
    if role == 'assistant':
        return assistant_text(payload)
    return None
