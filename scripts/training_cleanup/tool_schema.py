"""OpenAI-compatible tool-call shape validation."""


def valid_calls(value: object) -> bool:
    """Return whether a value is a non-empty list of typed function calls."""
    return (
        isinstance(value, list)
        and bool(value)
        and all(_valid(call) for call in value)
    )


def _valid(value: object) -> bool:
    if not isinstance(value, dict):
        return False
    function = value.get('function')
    return (
        isinstance(value.get('id'), str)
        and value.get('type') == 'function'
        and isinstance(function, dict)
        and isinstance(function.get('name'), str)
        and isinstance(function.get('arguments'), str)
    )
