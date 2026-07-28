"""Memory-bounded supervised fine-tuning configuration."""

import os

from pathlib import Path

from trl import SFTConfig

from .attention import flash_available
from .checkpoint_policy import CHECKPOINTING
from .constants import SEED, WARMUP_STEPS
from .distributed import accumulation_steps
from .precision import supports_bf16
from .sequence_length import resolve
from .throughput import settings as throughput_settings


def build(output: Path, epochs: float, masked: bool = False) -> SFTConfig:
    """Return deterministic settings scaled to the active GPU precision."""
    bf16 = supports_bf16()
    length = resolve(os.environ.get('CODETETHER_MAX_LENGTH'))
    return SFTConfig(
        output_dir=str(output),
        num_train_epochs=epochs,
        **throughput_settings(flash_available()),
        per_device_train_batch_size=1,
        per_device_eval_batch_size=1,
        gradient_accumulation_steps=accumulation_steps(),
        learning_rate=2e-4,
        lr_scheduler_type='cosine',
        warmup_steps=WARMUP_STEPS,
        weight_decay=0.01,
        fp16=not bf16,
        bf16=bf16,
        tf32=False,
        gradient_checkpointing=True,
        gradient_checkpointing_kwargs={'use_reentrant': False},
        ddp_find_unused_parameters=False,
        max_grad_norm=0.3,
        optim='paged_adamw_8bit',
        logging_steps=10,
        **CHECKPOINTING,
        report_to='none',
        seed=SEED,
        data_seed=SEED,
        max_length=length,
        dataset_text_field=None if masked else 'text',
    )
    # `packing` is supplied by throughput_settings above; repeating it here
    # raised TypeError: got multiple values for keyword argument 'packing'.
