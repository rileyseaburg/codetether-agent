"""Split the ingested corpus into leakage-free train and validation sets."""

import argparse
import hashlib
import json

from pathlib import Path


def main() -> None:
    """Assign whole conversations to splits by stable digest bucket."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--corpus', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--validation-bucket', default='00')
    values = parser.parse_args()
    values.output.mkdir(parents=True, exist_ok=True)
    train = values.output / 'train.jsonl'
    validation = values.output / 'validation.jsonl'
    counts = {'train': 0, 'validation': 0}
    with (
        values.corpus.open() as source,
        train.open('w') as train_out,
        validation.open('w') as validation_out,
    ):
        for line in source:
            record = json.loads(line)
            key = str(record['metadata']['sha256'])
            held = _bucket(key).startswith(values.validation_bucket)
            target = validation_out if held else train_out
            target.write(line)
            counts['validation' if held else 'train'] += 1
    report = {
        'train_conversations': counts['train'],
        'validation_conversations': counts['validation'],
        'validation_bucket': values.validation_bucket,
        'split_key': 'conversation sha256 prefix',
        'leakage': 'whole conversations never span splits',
    }
    path = values.output / 'split-manifest.json'
    path.write_text(json.dumps(report, indent=2, sort_keys=True) + '\n')
    print(json.dumps(report, sort_keys=True))


def _bucket(key: str) -> str:
    return hashlib.sha256(key.encode()).hexdigest()


if __name__ == '__main__':
    main()
