"""Score tool calls against the schema the agent actually exposes.

An untuned Qwen3.5-9B already emits syntactically valid calls for 7 of 8
probe prompts, measured at 0.875 on the deployment GPU. A syntactic rate
therefore cannot distinguish a trained adapter from the base model. What
training must add is use of *our* tool names and parameters.
"""

import re


CALL_PATTERN = re.compile(r'<function=([A-Za-z0-9_]+)>')
PARAM_PATTERN = re.compile(r'<parameter=([A-Za-z0-9_]+)>')

PLACEHOLDER_NAMES = frozenset({'example_function_name'})
"""Emitted by the chat template's instruction block, not a real tool."""


def called_tools(text: str) -> list[str]:
    """Return real tool names invoked in a completion."""
    return [
        name
        for name in CALL_PATTERN.findall(text)
        if name not in PLACEHOLDER_NAMES
    ]


def called_parameters(text: str) -> list[str]:
    """Return parameter names supplied in a completion."""
    return PARAM_PATTERN.findall(text)
