"""Tool schemas rendered into training prompts.

Inference always injects a `<tools>` block via the chat template. Training
text must contain the same block or the model learns to echo it as content.
"""

SCHEMAS: list[dict[str, object]] = [
    {
        'type': 'function',
        'function': {
            'name': 'read',
            'description': 'Read a file from the repository',
            'parameters': {
                'type': 'object',
                'properties': {'path': {'type': 'string'}},
                'required': ['path'],
            },
        },
    },
    {
        'type': 'function',
        'function': {
            'name': 'bash',
            'description': 'Run a shell command',
            'parameters': {
                'type': 'object',
                'properties': {'command': {'type': 'string'}},
                'required': ['command'],
            },
        },
    },
]
