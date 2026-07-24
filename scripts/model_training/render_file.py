"""Render bounded chat-template text for supervised fine-tuning."""

import hashlib
import json
from pathlib import Path


def build(
    source: Path, target: Path, tokenizer: object, limit: int
) -> dict[str, object]:
    """Render records and reject examples exceeding the token limit."""
    digest = hashlib.sha256()
    included = excluded = token_total = 0
    maximum = 0
    with target.open('wb') as output:
        for raw in source.read_text().splitlines():
            value = json.loads(raw)
            text = tokenizer.apply_chat_template(
                value['messages'], tokenize=False, add_generation_prompt=False
            )
            tokens = len(tokenizer(text, add_special_tokens=False)['input_ids'])
            if tokens > limit:
                excluded += 1
                continue
            record = {
                'text': text,
                'tokens': tokens,
                'metadata': value['metadata'],
            }
            line = _line(record)
            output.write(line)
            digest.update(line)
            included += 1
            token_total += tokens
            maximum = max(maximum, tokens)
    return {
        'source': str(source),
        'path': str(target),
        'included': included,
        'excluded_over_limit': excluded,
        'tokens': token_total,
        'mean_tokens': round(token_total / included, 2),
        'max_tokens': maximum,
        'bytes': target.stat().st_size,
        'sha256': digest.hexdigest(),
    }


def _line(value: dict[str, object]) -> bytes:
    return (
        json.dumps(value, sort_keys=True, separators=(',', ':')) + '\n'
    ).encode()
