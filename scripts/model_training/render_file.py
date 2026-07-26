"""Render bounded chat-template text for supervised fine-tuning."""

import hashlib
import json

from pathlib import Path

from .render_summary import Totals, summarize
from .render_tools import collect


def build(
    source: Path, target: Path, tokenizer: object, limit: int
) -> dict[str, object]:
    """Render records and reject examples exceeding the token limit."""
    digest = hashlib.sha256()
    totals = Totals(0, 0, 0, 0, 0)
    with target.open('wb') as output:
        for raw in source.read_text().splitlines():
            value = json.loads(raw)
            tools = collect(value['messages'])
            text = tokenizer.apply_chat_template(
                value['messages'],
                tools=tools or None,
                tokenize=False,
                add_generation_prompt=False,
            )
            tokens = len(tokenizer(text, add_special_tokens=False)['input_ids'])
            if tokens > limit:
                totals.excluded += 1
                continue
            line = _line(
                {
                    'text': text,
                    'tokens': tokens,
                    'metadata': value['metadata'],
                }
            )
            output.write(line)
            digest.update(line)
            totals.included += 1
            totals.tokens += tokens
            totals.maximum = max(totals.maximum, tokens)
            totals.with_tools += 1 if tools else 0
    return summarize(source, target, totals, digest.hexdigest())


def _line(value: dict[str, object]) -> bytes:
    return (
        json.dumps(value, sort_keys=True, separators=(',', ':')) + '\n'
    ).encode()
