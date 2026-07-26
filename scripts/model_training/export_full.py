"""Export the full governed corpus by message-digest shard."""

import argparse
import json

from pathlib import Path

from .export_shard import BUCKETS
from .shard_plan import ShardPlan
from .shard_writer import write_split
from .trino_cli import query


def main() -> None:
    """Write sharded train and validation splits with leakage evidence."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--snapshot', type=int, required=True)
    parser.add_argument('--per-shard', type=int, default=4000)
    parser.add_argument('--buckets', default=''.join(BUCKETS))
    values = parser.parse_args()
    values.output.mkdir(parents=True, exist_ok=True)
    buckets = list(values.buckets)
    plan = ShardPlan(
        values.output, buckets, values.per_shard, values.snapshot, query
    )
    train = write_split('train', plan)
    validation = write_split('validation', plan)
    overlap = train['correlations'] & validation['correlations']
    if overlap:
        raise ValueError(f'correlation leakage: {len(overlap)}')
    evidence = {
        'snapshot_id': values.snapshot,
        'buckets': values.buckets,
        'train': _summary(train),
        'validation': _summary(validation),
        'correlation_overlap': 0,
    }
    path = values.output / 'export-full-manifest.json'
    path.write_text(json.dumps(evidence, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'manifest': str(path), **evidence}, sort_keys=True))


def _summary(split: dict[str, object]) -> dict[str, object]:
    return {
        'records': split['records'],
        'correlations': len(split['correlations']),
        'path': split['path'],
        'sha256': split['sha256'],
    }


if __name__ == '__main__':
    main()
