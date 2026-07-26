"""Verify host memory and disk before merging a large adapter.

Merging dequantizes the base model on the CPU, so a 30B model needs roughly
61 GB of RAM in 16-bit precision. Failing here is far cheaper than dying
partway through writing shards.
"""

import shutil

from pathlib import Path

from .vram_plan import LARGE_MODEL


BYTES_PER_PARAM = 2
LARGE_PARAMS_B = 30.5
SMALL_PARAMS_B = 4.0
OVERHEAD_GB = 6.0


def required_ram_gb(model: str) -> float:
    """Return the RAM needed to dequantize and merge a model."""
    params = LARGE_PARAMS_B if model == LARGE_MODEL else SMALL_PARAMS_B
    return params * BYTES_PER_PARAM + OVERHEAD_GB


def available_ram_gb() -> float:
    """Return usable memory from MemAvailable, in gigabytes."""
    for line in Path('/proc/meminfo').read_text().splitlines():
        if line.startswith('MemAvailable:'):
            return int(line.split()[1]) * 1024 / 1e9
    return 0.0


def verify(model: str, output: Path) -> dict[str, object]:
    """Raise when memory or disk cannot complete the merge."""
    ram = available_ram_gb()
    needed = required_ram_gb(model)
    free = shutil.disk_usage(output.parent).free / 1e9
    budget = {
        'available_ram_gb': round(ram, 1),
        'required_ram_gb': needed,
        'free_disk_gb': round(free, 1),
    }
    if ram < needed:
        raise SystemExit(f'insufficient RAM to merge: {budget}')
    if free < needed:
        raise SystemExit(f'insufficient disk to write merged model: {budget}')
    return budget
