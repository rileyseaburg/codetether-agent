"""Immutable base-model and training hyperparameters.

Pinned so a run can be reproduced exactly. Model identity lives in
`base_models`, which documents why the deployment target's 8 GB of VRAM
decides the size.
"""

from model_training.base_models import (
    BASE_MODEL,
    BASE_REVISION,
    SMALL_MODEL,
    SMALL_REVISION,
)
from model_training.lora_params import (
    LORA_ALPHA,
    LORA_DROPOUT,
    LORA_RANK,
    TARGET_MODULES,
)
from model_training.schedule import (
    ACCUMULATION_STEPS,
    EVAL_STEPS,
    MAX_EVAL_SAMPLES,
    WARMUP_STEPS,
)


MAX_LENGTH = 8192
SEED = 42

__all__ = [
    'ACCUMULATION_STEPS',
    'BASE_MODEL',
    'BASE_REVISION',
    'EVAL_STEPS',
    'LORA_ALPHA',
    'LORA_DROPOUT',
    'LORA_RANK',
    'MAX_EVAL_SAMPLES',
    'MAX_LENGTH',
    'SEED',
    'SMALL_MODEL',
    'SMALL_REVISION',
    'TARGET_MODULES',
    'WARMUP_STEPS',
]
