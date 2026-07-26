"""Resolve the training sequence length for the active device.

The device plan decides sequence length, so a fixed constant would silently
override a smaller GPU's safe budget. An explicit override wins when set.
"""

import torch

from .constants import MAX_LENGTH
from .vram_plan import plan


def resolve(override: str | None = None) -> int:
    """Return the sequence length for this run."""
    if override:
        return int(override)
    if not torch.cuda.is_available():
        return MAX_LENGTH
    total = torch.cuda.get_device_properties(0).total_memory / 1e9
    return int(plan(total)['max_length'])
