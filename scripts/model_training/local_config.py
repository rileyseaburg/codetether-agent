"""Training configuration for a memory-constrained local GPU.

An 8 GB card cannot hold the 8,192-token configuration used on rented A100s.
This keeps the same objective and masking while shrinking sequence length and
disabling the paged optimizer, which requires more headroom than remains
after the 4-bit base and LoRA adapters are resident.
"""

import os

from pathlib import Path

from trl import SFTConfig

from .constants import SEED
from .local_schedule import LOCAL_CHECKPOINTING


LOCAL_MAX_LENGTH = 2048
"""Measured median prompt is 2,719 characters, roughly 700 tokens."""

LOCAL_ACCUMULATION = 16


def build_local(output: Path, epochs: float) -> SFTConfig:
    """Return settings that fit an 8 GB device."""
    length = int(os.environ.get('CODETETHER_MAX_LENGTH', LOCAL_MAX_LENGTH))
    return SFTConfig(
        output_dir=str(output),
        num_train_epochs=epochs,
        per_device_train_batch_size=1,
        per_device_eval_batch_size=1,
        gradient_accumulation_steps=LOCAL_ACCUMULATION,
        learning_rate=2e-4,
        lr_scheduler_type='cosine',
        warmup_steps=20,
        weight_decay=0.01,
        fp16=True,
        bf16=False,
        gradient_checkpointing=True,
        gradient_checkpointing_kwargs={'use_reentrant': False},
        max_grad_norm=0.3,
        optim='adamw_torch',
        logging_steps=5,
        **LOCAL_CHECKPOINTING,
        report_to='none',
        seed=SEED,
        data_seed=SEED,
        max_length=length,
        packing=False,
        padding_free=False,
    )
