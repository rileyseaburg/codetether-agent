"""Compute label-masking statistics for rendered prompt/completion pairs."""

import itertools
import json

from pathlib import Path


def audit(
    pairs: Path, tokenizer: object, sample: int, max_length: int
) -> dict[str, object]:
    """Return how much of each sequence carries training signal."""
    counted = 0
    prompt_total = 0
    completion_total = 0
    fully_masked = 0
    truncated = 0
    with pairs.open() as handle:
        for line in itertools.islice(handle, sample):
            record = json.loads(line)
            prompt = _count(tokenizer, record['prompt'])
            completion = _count(tokenizer, record['completion'])
            counted += 1
            prompt_total += prompt
            completion_total += completion
            if prompt + completion > max_length:
                truncated += 1
            if prompt >= max_length:
                fully_masked += 1
    total = prompt_total + completion_total
    return {
        'pairs_sampled': counted,
        'max_length': max_length,
        'mean_prompt_tokens': round(prompt_total / max(counted, 1), 1),
        'mean_completion_tokens': round(completion_total / max(counted, 1), 1),
        'supervised_fraction': round(completion_total / max(total, 1), 4),
        'fully_masked': fully_masked,
        'truncated': truncated,
        'rule': 'loss applies to completion tokens only',
    }


def _count(tokenizer: object, text: str) -> int:
    """Return the token count for one string."""
    return len(tokenizer(text, add_special_tokens=False)['input_ids'])
