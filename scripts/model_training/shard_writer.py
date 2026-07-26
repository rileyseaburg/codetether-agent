"""Accumulate sharded Trino results into one deterministic JSONL split."""

import hashlib
import json

from pathlib import Path

from .export_query import build
from .shard_plan import ShardPlan


def write_split(split: str, plan: ShardPlan) -> dict[str, object]:
    """Query every shard and append its rows to a single split file."""
    target = Path(plan.output) / f'{split}.jsonl'
    digest = hashlib.sha256()
    correlations: set[str] = set()
    records = 0
    with target.open('wb') as handle:
        for bucket in plan.buckets:
            sql = build(split, plan.per_shard, plan.snapshot, False, bucket)
            for row in plan.runner(sql):
                payload = {
                    'messages': json.loads(str(row['messages_json'])),
                    'metadata': {
                        'sample_id': row['sample_id'],
                        'correlation_id': row['correlation_id'],
                        'message_sha256': row['message_sha'],
                    },
                }
                line = (json.dumps(payload, sort_keys=True) + '\n').encode()
                handle.write(line)
                digest.update(line)
                correlations.add(str(row['correlation_id']))
                records += 1
    return {
        'path': str(target),
        'records': records,
        'correlations': correlations,
        'sha256': digest.hexdigest(),
    }
