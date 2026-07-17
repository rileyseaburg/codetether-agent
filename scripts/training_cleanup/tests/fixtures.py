"""Small CodeTether bus-record fixtures."""

import json


def record(
    role: str, kind: str, line: int, **values: object
) -> dict[str, object]:
    """Build one source record with deterministic provenance."""
    correlation = values.pop('correlation_id', 'turn-1')
    return {
        'role': role,
        **values,
        'metadata': {
            'bus_kind': kind,
            'correlation_id': correlation,
            'sender_id': 'build',
            'timestamp': f'2026-07-17T20:07:{line:02d}Z',
        },
    }


def tool_pair(line: int) -> list[dict[str, object]]:
    """Build a valid assistant call and tool response pair."""
    identifier = f'call-{line}'
    call = {
        'id': identifier,
        'type': 'function',
        'function': {'name': 'exec_command', 'arguments': '{}'},
    }
    return [
        record(
            'assistant',
            'tool_request_batch',
            line,
            content=None,
            tool_calls=[call],
        ),
        record(
            'tool',
            'tool_response',
            line + 1,
            content='ok',
            tool_call_id=identifier,
        ),
    ]


def jsonl(values: list[object]) -> str:
    """Serialize fixture values as JSON Lines."""
    return '\n'.join(json.dumps(value) for value in values)
