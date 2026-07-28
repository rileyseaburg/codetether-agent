"""Split conversations into prompt/completion supervision pairs."""

from .render_tool_calls import render_calls


Message = dict[str, object]


def role(message: Message) -> str:
    """Return the message role."""
    return str(message['role'])


def pairs(messages: list[Message]) -> list[tuple[list[Message], Message]]:
    """Yield (context, assistant_reply) pairs for supervised training.

    Only assistant turns become completions, so loss is never applied to
    system, user, or tool tokens.
    """
    result: list[tuple[list[Message], Message]] = []
    for index, message in enumerate(messages):
        if role(message) != 'assistant':
            continue
        context = messages[:index]
        if not any(role(m) == 'user' for m in context):
            continue
        result.append((context, message))
    return result


def text_of(message: Message) -> str:
    """Return assistant prose followed by any emitted tool calls.

    Tool calls must be part of the completion or loss never covers them.
    Ingestion recorded 9,943 assistant messages with `tool_calls` that the
    earlier render discarded, leaving the corpus unable to teach our schema.
    """
    content = message.get('content')
    prose = _prose(content)
    calls = message.get('tool_calls')
    if isinstance(calls, list) and calls:
        rendered = render_calls(calls)
        return f'{prose}\n{rendered}' if prose.strip() else rendered
    return prose


def _prose(content: object) -> str:
    """Return the textual portion of a message body."""
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        parts = [
            str(p.get('text', ''))
            for p in content
            if isinstance(p, dict) and p.get('type') == 'text'
        ]
        return ''.join(parts)
    return ''
