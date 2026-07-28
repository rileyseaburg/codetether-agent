"""Distributed settings that preserve optimization semantics."""

import os

from .schedule import ACCUMULATION_STEPS


def world_size() -> int:
    """Return the number of torchrun workers."""
    return max(int(os.environ.get('WORLD_SIZE', '1')), 1)


def local_rank() -> int:
    """Return the CUDA device assigned to this worker."""
    return max(int(os.environ.get('LOCAL_RANK', '0')), 0)


def accumulation_steps() -> int:
    """Keep effective batch size stable as GPU count increases."""
    return max(ACCUMULATION_STEPS // world_size(), 1)


def is_primary() -> bool:
    """Return whether this worker owns generated artifacts."""
    return int(os.environ.get('RANK', '0')) == 0
