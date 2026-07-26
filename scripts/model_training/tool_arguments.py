"""Normalize tool-call arguments for chat-template rendering.

Qwen chat templates iterate tool arguments as a mapping, so JSON strings
captured from transcripts must be parsed before rendering.
"""

import json


def mapping(arguments: object) -> dict[str, object]:
    """Return tool arguments as a mapping."""
    if isinstance(arguments, dict):
        return arguments
    if isinstance(arguments, str):
        try:
            parsed = json.loads(arguments)
        except ValueError:
            return {'input': arguments}
        return parsed if isinstance(parsed, dict) else {'input': parsed}
    return {}
