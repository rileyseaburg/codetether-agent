"""Upload canonical local sessions into the immutable training source."""

import os

from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

from .client import create
from .discovery import canonical
from .objects import chunks
from .upload import one


def main() -> None:
    """Discover, convert, and idempotently upload VM sessions."""
    bucket = os.environ.get('TRAINING_BUCKET', 'codetether-training')
    client = create()
    paths = canonical(Path(os.environ.get('VM_HOME', '/vm')))

    def upload(path: Path) -> Counter[str]:
        result: Counter[str] = Counter(sessions=1)
        for key, payload in chunks(path):
            result[one(client, bucket, key, payload)] += 1
            result['records'] += payload.count(b'\n')
            result['bytes'] += len(payload)
        return result

    total: Counter[str] = Counter()
    workers = int(os.environ.get('IMPORT_WORKERS', '8'))
    with ThreadPoolExecutor(max_workers=workers) as pool:
        for count, result in enumerate(pool.map(upload, paths.values()), 1):
            total.update(result)
            if count % 500 == 0:
                print(
                    f'progress={count}/{len(paths)} '
                    f'uploaded={total["uploaded"]}',
                    flush=True,
                )
    total['discovered_sessions'] = len(paths)
    print(dict(total), flush=True)


if __name__ == '__main__':
    main()
