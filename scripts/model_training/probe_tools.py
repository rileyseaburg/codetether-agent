"""Tool schema supplied to behaviour probes.

Mirrors the tools the agent actually exposes, because a model only emits a
tool call when the rendered prompt advertises callable functions.
"""


def _function(name: str, description: str, field: str) -> dict[str, object]:
    """Return one function entry with a single required string parameter."""
    return {
        'type': 'function',
        'function': {
            'name': name,
            'description': description,
            'parameters': {
                'type': 'object',
                'properties': {field: {'type': 'string'}},
                'required': [field],
            },
        },
    }


TOOL_SCHEMA = [
    _function('read', 'Read the contents of a file', 'path'),
    _function('bash', 'Run a shell command', 'command'),
    _function('grep', 'Search files for a pattern', 'pattern'),
]
"""Three tools covering the probe prompts: inspection, execution, search."""
