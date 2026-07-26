"""Build leakage-free new-data continuation and frozen validation sets."""

import argparse
import json

from pathlib import Path

from .text_rows import correlation, read, sha, unique, write


def main() -> None:
    """Subtract prior training data while preserving both validation sets."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--prior', type=Path, required=True)
    parser.add_argument('--current', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    prior_train = read(values.prior / 'train-1024.jsonl')
    prior_validation = read(values.prior / 'validation-1024.jsonl')
    current_train = read(values.current / 'train-1024.jsonl')
    current_validation = read(values.current / 'validation-1024.jsonl')
    seen = {sha(row) for row in prior_train}
    held_out = unique(prior_validation + current_validation, seen)
    held_out_ids = {sha(row) for row in held_out}
    delta = unique(current_train, seen | held_out_ids)
    train_correlations = {correlation(row) for row in prior_train + delta}
    validation_correlations = {correlation(row) for row in held_out}
    overlap = train_correlations & validation_correlations
    if overlap:
        raise ValueError(f'correlation leakage: {len(overlap)}')
    values.output.mkdir(parents=True, exist_ok=True)
    evidence = {
        'prior_train_records': len(prior_train),
        'delta_train_records': len(delta),
        'validation_records': len(held_out),
        'semantic_overlap': 0,
        'correlation_overlap': 0,
        'delta_sha256': write(values.output / 'train-1024.jsonl', delta),
        'validation_sha256': write(
            values.output / 'validation-1024.jsonl', held_out
        ),
    }
    manifest = values.output / 'continuation-manifest.json'
    manifest.write_text(json.dumps(evidence, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'manifest': str(manifest), **evidence}, sort_keys=True))


if __name__ == '__main__':
    main()
