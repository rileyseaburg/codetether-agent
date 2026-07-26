"""Render prompt/completion pairs with inference-matched tool schemas."""

from .tool_schema import SCHEMAS
from .turn_split import pairs, text_of
from .turn_window import stride, window


def render(
    messages: list[dict[str, object]], tokenizer: object, with_tools: bool
) -> list[dict[str, str]]:
    """Return bounded prompt/completion records for assistant turns."""
    selected = pairs(messages)
    step = stride(len(selected))
    records: list[dict[str, str]] = []
    for index, (context, reply) in enumerate(selected):
        if index % step:
            continue
        completion = text_of(reply)
        if not completion.strip():
            continue
        prompt = _prompt(window(context), tokenizer, with_tools)
        records.append(
            {
                'prompt': prompt,
                'completion': f'{completion}<|im_end|>',
            }
        )
    return records


def _prompt(
    context: list[dict[str, object]], tokenizer: object, with_tools: bool
) -> str:
    """Render the context with a generation prompt appended."""
    kwargs: dict[str, object] = {
        'tokenize': False,
        'add_generation_prompt': True,
    }
    if with_tools:
        kwargs['tools'] = SCHEMAS
    return str(tokenizer.apply_chat_template(context, **kwargs))
