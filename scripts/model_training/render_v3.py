"""Render the v3 masked, tool-aware, short-primed training corpus."""

import argparse
import json

from pathlib import Path

from transformers import AutoTokenizer

from .constants import BASE_MODEL, BASE_REVISION
from .pair_dedupe import prune
from .render_pair_file import build
from .short_primer import append


def main() -> None:
    """Render masked pairs and append short instruction primers."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--directory', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--max-tokens', type=int, default=8192)
    parser.add_argument('--short-repeats', type=int, default=12)
    values = parser.parse_args()
    tokenizer = AutoTokenizer.from_pretrained(
        BASE_MODEL, revision=BASE_REVISION
    )
    values.output.mkdir(parents=True, exist_ok=True)
    evidence = {
        'base_model': BASE_MODEL,
        'base_revision': BASE_REVISION,
        'loss_masking': 'prompt-completion; loss on assistant tokens only',
        'train': build(
            values.directory / 'train.jsonl',
            values.output / 'train-pairs.jsonl',
            tokenizer,
            values.max_tokens,
        ),
        'validation': build(
            values.directory / 'validation.jsonl',
            values.output / 'validation-pairs.jsonl',
            tokenizer,
            values.max_tokens,
        ),
        'short_primers': append(
            values.output / 'train-pairs.jsonl', tokenizer, values.short_repeats
        ),
    }
    evidence['leakage'] = prune(
        values.output / 'train-pairs.jsonl',
        values.output / 'validation-pairs.jsonl',
    )
    path = values.output / 'render-v3-manifest.json'
    path.write_text(json.dumps(evidence, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'manifest': str(path), **evidence}, sort_keys=True))


if __name__ == '__main__':
    main()
