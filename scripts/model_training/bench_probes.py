"""Run tool-call and code-correction probes against a local server."""

from .bench_cases import CODE_CASES, TOOL_PROMPTS, TOOL_SCHEMA
from .bench_client import chat


def tool_probes(base_url: str, model: str) -> list[dict[str, object]]:
    """Check whether the model emits structured tool calls."""
    results: list[dict[str, object]] = []
    for prompt in TOOL_PROMPTS:
        message = chat(
            base_url,
            model,
            [{'role': 'user', 'content': prompt}],
            TOOL_SCHEMA,
        )
        calls = message.get('tool_calls') or []
        results.append(
            {
                'prompt': prompt,
                'emitted_tool_calls': bool(calls),
                'tool_name': calls[0]['function']['name'] if calls else None,
                'text': str(message.get('content') or '')[:200],
            }
        )
    return results


def code_probes(base_url: str, model: str) -> list[dict[str, object]]:
    """Check whether the model produces the expected correction token."""
    results: list[dict[str, object]] = []
    for prompt, expected in CODE_CASES:
        message = chat(base_url, model, [{'role': 'user', 'content': prompt}])
        text = str(message.get('content') or '')
        results.append(
            {
                'prompt': prompt[:70],
                'expected_token': expected,
                'passed': expected in text,
                'text': text[:300],
            }
        )
    return results
