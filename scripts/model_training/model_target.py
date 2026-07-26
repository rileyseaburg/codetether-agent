"""Resolve which base model this device should actually train.

The trainer previously loaded a hardcoded model regardless of the device
plan, so an A100 40 GB attempted the 30B mixture-of-experts model and hit
CUDA OOM at 39.47 GiB. Selection now follows measured capacity.
"""

import os

import torch

from .constants import BASE_MODEL, BASE_REVISION, SMALL_MODEL, SMALL_REVISION
from .vram_plan import LARGE_MODEL, plan


REVISIONS = {
    LARGE_MODEL: BASE_REVISION,
    SMALL_MODEL: SMALL_REVISION,
}


def resolve_target() -> tuple[str, str | None]:
    """Return the model identifier and pinned revision for this device."""
    override = os.environ.get('CODETETHER_BASE_MODEL')
    if override:
        return override, REVISIONS.get(override)
    if not torch.cuda.is_available():
        return BASE_MODEL, BASE_REVISION
    total = torch.cuda.get_device_properties(0).total_memory / 1e9
    model = str(plan(total)['recommended_model'])
    return model, REVISIONS.get(model)
