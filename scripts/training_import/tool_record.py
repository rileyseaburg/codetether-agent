"""Trainer-facing records for native tool results."""


def build(
    part: dict[str, object],
    names: dict[str, str],
    metadata: dict[str, object],
) -> dict[str, object]:
    """Return one tool response with its original call identifier."""
    identifier = str(part.get('tool_call_id') or '')
    return {
        'role': 'tool',
        'content': str(part.get('content') or ''),
        'tool_call_id': identifier,
        'name': names.get(identifier, 'unknown'),
        'metadata': metadata,
    }
