"""Pin a tokenizer and render GPU-bounded SFT datasets."""

import argparse
import json
from pathlib import Path

from transformers import AutoTokenizer

from .render_file import build


MODEL = 'Qwen/Qwen2.5-Coder-1.5B-Instruct'
REVISION = '2e1fd397ee46e1388853d2af2c993145b0f1098a'


def main() -> None:
    """Render train and validation text using the pinned base template."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--directory', type=Path, required=True)
    parser.add_argument('--max-tokens', type=int, default=1024)
    values = parser.parse_args()
    tokenizer = AutoTokenizer.from_pretrained(MODEL, revision=REVISION)
    evidence = {
        'base_model': MODEL,
        'base_revision': REVISION,
        'tokenizer_class': type(tokenizer).__name__,
        'vocab_size': len(tokenizer),
        'max_tokens': values.max_tokens,
        'train': build(
            values.directory / 'train.jsonl',
            values.directory / 'train-1024.jsonl',
            tokenizer,
            values.max_tokens,
        ),
        'validation': build(
            values.directory / 'validation.jsonl',
            values.directory / 'validation-1024.jsonl',
            tokenizer,
            values.max_tokens,
        ),
    }
    path = values.directory / 'render-manifest.json'
    path.write_text(json.dumps(evidence, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'manifest': str(path), **evidence}, sort_keys=True))


if __name__ == '__main__':
    main()
