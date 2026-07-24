"""Argument declarations for the Spark training cleanup job."""

from argparse import ArgumentParser


def parser() -> ArgumentParser:
    """Build the cleanup argument parser; omission of apply means dry-run."""
    value = ArgumentParser(
        description='Clean CodeTether training JSONL with Spark'
    )
    value.add_argument('--endpoint')
    value.add_argument('--bucket', default='codetether-training')
    value.add_argument('--source-prefix', default='training/v2')
    value.add_argument('--source-before')
    value.add_argument('--catalog', default='iceberg')
    value.add_argument('--namespace', default='codetether')
    value.add_argument('--table-prefix', default='training')
    value.add_argument('--run-id')
    value.add_argument('--max-content-chars', type=int, default=32_768)
    value.add_argument('--max-sample-chars', type=int, default=65_536)
    value.add_argument('--max-sample-messages', type=int, default=96)
    value.add_argument('--min-partitions', type=int, default=64)
    value.add_argument('--apply', action='store_true')
    value.add_argument('--reprocess', action='store_true')
    return value
