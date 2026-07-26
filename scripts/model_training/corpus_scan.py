"""Locate transcript stores beneath a root directory."""

import os

from pathlib import Path


STORE_NAMES = {'sessions', 'rollouts', 'transcripts', 'chat_events'}
SKIP = {
    'node_modules',
    'target',
    '.git',
    'venv',
    '.venv',
    'site-packages',
    'hf-cache',
    'models',
    '.vscode-server-insiders',
    'google-cloud-sdk',
}


def scan_root(root: Path, min_bytes: int) -> list[dict[str, object]]:
    """Return per-directory transcript counts, skipping vendored trees."""
    stores: list[dict[str, object]] = []
    for current, directories, files in os.walk(root, topdown=True):
        directories[:] = [d for d in directories if d not in SKIP]
        name = Path(current).name
        if name not in STORE_NAMES and '.codetether' not in current:
            continue
        count = 0
        size = 0
        for entry in files:
            if not entry.endswith(('.jsonl', '.json')):
                continue
            try:
                stat = Path(current, entry).stat()
            except OSError:
                continue
            if stat.st_size < min_bytes:
                continue
            count += 1
            size += stat.st_size
        if count:
            stores.append({'path': current, 'files': count, 'bytes': size})
    return stores
