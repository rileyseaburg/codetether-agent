"""Choose a trainable base model from available VRAM.

Selection is bounded by the deployment target, an RTX 2080 Super with 8 GB
of VRAM, not by the training device. A model that trains comfortably on an
A100 but needs 15.3 GB at Q4_K_M cannot be served, so larger bases are not
offered regardless of training capacity.
"""

from model_training.constants import BASE_MODEL, MAX_LENGTH, SMALL_MODEL
from model_training.memory_budget import estimate


TRAIN_9B_GB = 22.0
TRAIN_4B_GB = 14.0
"""Below this a 4B run must shorten sequences to fit activations."""
BASE_PARAMS = 9.7
SMALL_PARAMS = 4.7


def plan(gigabytes: float) -> dict[str, object]:
    """Return the recommended model and sequence length for a device."""
    if gigabytes >= TRAIN_9B_GB:
        return _entry(BASE_MODEL, BASE_PARAMS, MAX_LENGTH, 'a100-or-better')
    if gigabytes >= TRAIN_4B_GB:
        return _entry(SMALL_MODEL, SMALL_PARAMS, MAX_LENGTH, 'l4-or-4090')
    return _entry(SMALL_MODEL, SMALL_PARAMS, 2048, 't4-class')


def _entry(
    model: str, params: float, length: int, tier: str
) -> dict[str, object]:
    return {
        'recommended_model': model,
        'max_length': length,
        'tier': tier,
        'mixture_of_experts': False,
        'estimated': estimate(params, length, False),
        'serves_on_8gb': round(params * 0.55 + 1.7, 1),
    }
