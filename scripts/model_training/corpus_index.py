"""Index every agent transcript store on this host.

Session data is not centralized: each project directory keeps its own
`.codetether-agent/sessions` store alongside the global Codex rollouts, so a
single-directory survey undercounts the corpus by orders of magnitude.
"""

import argparse
import json

from pathlib import Path

from .corpus_scan import scan_root


PATTERNS = ('*.jsonl', '*.json')


def main() -> None:
    """Write an incremental index of transcript stores and their sizes."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--min-bytes', type=int, default=4096)
    values = parser.parse_args()
    stores = scan_root(values.root, values.min_bytes)
    total_files = sum(int(s['files']) for s in stores)
    total_bytes = sum(int(s['bytes']) for s in stores)
    report = {
        'root': str(values.root),
        'stores': len(stores),
        'files': total_files,
        'bytes': total_bytes,
        'gigabytes': round(total_bytes / 1e9, 2),
        'largest': sorted(stores, key=lambda s: -int(s['bytes']))[:25],
    }
    values.output.parent.mkdir(parents=True, exist_ok=True)
    values.output.write_text(
        json.dumps(report, indent=2, sort_keys=True) + '\n'
    )
    paths = sorted(str(store['path']) for store in stores)
    stores_path = values.output.parent / 'host-stores.json'
    stores_path.write_text(json.dumps(paths, indent=1) + '\n')
    print(
        json.dumps(
            {
                'stores': report['stores'],
                'files': report['files'],
                'gigabytes': report['gigabytes'],
            }
        )
    )


if __name__ == '__main__':
    main()
