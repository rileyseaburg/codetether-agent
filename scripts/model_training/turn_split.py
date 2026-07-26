"""Split conversations into prompt/completion supervision pairs."""

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
    """Return assistant text content, empty when absent."""
    content = message.get('content')
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
