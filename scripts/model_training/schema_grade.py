"""Judge whether a tool call matches our schema.

Distinguishes a model that knows our tools from one that invents plausible
names, which a syntactic tool-call rate cannot do.
"""

from .tool_call_parse import called_parameters, called_tools


def grade(text: str, schema: list[dict[str, object]]) -> dict[str, bool]:
    """Return schema-conformance facts for one generated completion."""
    tools = called_tools(text)
    known = _known_names(schema)
    names_valid = bool(tools) and all(name in known for name in tools)
    return {
        'emitted': bool(tools),
        'known_tool': names_valid,
        'invented_tool': bool(tools) and not names_valid,
        'params_valid': names_valid and _params_ok(text, tools, schema),
    }


def _known_names(schema: list[dict[str, object]]) -> set[str]:
    """Return every tool name the schema advertises."""
    return {str(entry['function']['name']) for entry in schema}


def _params_ok(
    text: str, tools: list[str], schema: list[dict[str, object]]
) -> bool:
    """Return whether supplied parameters exist on the called tools."""
    allowed: set[str] = set()
    for entry in schema:
        function = entry['function']
        if str(function['name']) in tools:
            properties = function['parameters']['properties']
            allowed.update(properties)
    supplied = called_parameters(text)
    return bool(supplied) and all(name in allowed for name in supplied)
