"""Refuse to start when disk cannot hold the base model.

Qwen3-Coder-30B is roughly 60 GB across 16 shards. Discovering a full disk
at 90 percent of that download wastes an hour of GPU time, so the check runs
before any weights are fetched.
"""

import argparse
import json
import shutil

from pathlib import Path

from .vram_plan import LARGE_MODEL


LARGE_MODEL_GB = 62.0
SMALL_MODEL_GB = 9.0
HEADROOM_GB = 12.0
"""Checkpoints, the merged export, and the dataset also need room."""


def required_gb(model: str) -> float:
    """Return the disk needed for a model plus working headroom."""
    weights = LARGE_MODEL_GB if model == LARGE_MODEL else SMALL_MODEL_GB
    return weights + HEADROOM_GB


def main() -> None:
    """Verify free disk and exit non-zero when it is insufficient."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--path', type=Path, default=Path('/content'))
    parser.add_argument('--model', default=LARGE_MODEL)
    values = parser.parse_args()
    usage = shutil.disk_usage(values.path)
    free = usage.free / 1e9
    needed = required_gb(values.model)
    report = {
        'free_gb': round(free, 1),
        'required_gb': needed,
        'model': values.model,
        'sufficient': free >= needed,
    }
    print(json.dumps(report, sort_keys=True))
    if not report['sufficient']:
        raise SystemExit(
            f'insufficient disk: {free:.1f} GB free, {needed} GB needed'
        )


if __name__ == '__main__':
    main()
