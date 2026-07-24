"""Typed subprocess adapter for the in-cluster Trino CLI."""

import json
import subprocess


def query(sql: str) -> list[dict[str, object]]:
    """Execute SQL and decode Trino's JSON-lines output."""
    command = [
        'kubectl',
        'exec',
        '-n',
        'codetether-data',
        'deploy/trino',
        '--',
        'trino',
        '--catalog',
        'iceberg',
        '--schema',
        'codetether',
        '--output-format',
        'JSON',
        '--execute',
        sql,
    ]
    result = subprocess.run(command, check=True, text=True, capture_output=True)
    return [json.loads(line) for line in result.stdout.splitlines() if line]
