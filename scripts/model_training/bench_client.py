"""Minimal OpenAI-compatible chat client for local benchmarking."""

import json
import urllib.request


def chat(
    base_url: str,
    model: str,
    messages: list[dict[str, object]],
    tools: list[dict[str, object]] | None = None,
    max_tokens: int = 200,
) -> dict[str, object]:
    """Send one non-streaming chat completion request."""
    payload: dict[str, object] = {
        'model': model,
        'messages': messages,
        'temperature': 0,
        'max_tokens': max_tokens,
    }
    if tools:
        payload['tools'] = tools
    request = urllib.request.Request(
        f'{base_url}/v1/chat/completions',
        data=json.dumps(payload).encode(),
        headers={'Content-Type': 'application/json'},
        method='POST',
    )
    with urllib.request.urlopen(request, timeout=300) as response:
        body = json.load(response)
    message = body['choices'][0]['message']
    if not isinstance(message, dict):
        raise ValueError('message must be an object')
    return message
