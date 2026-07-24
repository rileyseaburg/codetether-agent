"""Export reproducible train and validation data from Iceberg."""

import json

from .dataset_file import write
from .export_args import parse
from .export_query import build, latest_snapshot, manifest
from .export_verify import splits
from .trino_cli import query


def main() -> None:
    """Pin a snapshot, export both splits, and emit an audit manifest."""
    settings = parse()
    settings.output.mkdir(parents=True, exist_ok=True)
    snapshot = int(query(latest_snapshot())[0]['snapshot_id'])
    train_rows = query(build('train', settings.train_limit, snapshot))
    validation_rows = query(
        build('validation', settings.validation_limit, snapshot)
    )
    train_path = settings.output / 'train.jsonl'
    validation_path = settings.output / 'validation.jsonl'
    evidence = {
        'source': {
            'catalog': 'iceberg',
            'namespace': 'codetether',
            'table': 'training_samples',
            'snapshot': query(manifest(snapshot))[0],
        },
        'selection': {
            'cleanup_version': 2,
            'message_chars': [64, 8192],
            'encrypted_reasoning_excluded': True,
            'deduplication': 'sha256(messages_json)',
            'validation_bucket': 'sha256(correlation_id)[0] == 0',
        },
        'train': write(train_path, train_rows),
        'validation': write(validation_path, validation_rows),
    }
    evidence['verification'] = splits(train_path, validation_path)
    output = settings.output / 'manifest.json'
    output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'manifest': str(output), **evidence}, sort_keys=True))


if __name__ == '__main__':
    main()
