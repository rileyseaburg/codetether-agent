"""Memory-bounded supervised fine-tuning configuration."""

import os

from pathlib import Path

from trl import SFTConfig

from .constants import SEED, WARMUP_STEPS
from .precision import supports_bf16
from .sequence_length import resolve


def build(output: Path, epochs: float, masked: bool = False) -> SFTConfig:
    """Return deterministic settings scaled to the active GPU precision."""
    bf16 = supports_bf16()
    length = resolve(os.environ.get('CODETETHER_MAX_LENGTH'))
    return SFTConfig(
        output_dir=str(output),
        num_train_epochs=epochs,
        per_device_train_batch_size=1,
        per_device_eval_batch_size=1,
        gradient_accumulation_steps=16,
        learning_rate=2e-4,
        lr_scheduler_type='cosine',
        warmup_steps=WARMUP_STEPS,
        weight_decay=0.01,
        fp16=not bf16,
        bf16=bf16,
        tf32=False,
        gradient_checkpointing=True,
        gradient_checkpointing_kwargs={'use_reentrant': False},
        max_grad_norm=0.3,
        optim='paged_adamw_8bit',
        logging_steps=10,
        eval_strategy='steps',
        eval_steps=250,
        save_strategy='steps',
        save_steps=250,
        save_total_limit=3,
        load_best_model_at_end=True,
        metric_for_best_model='eval_loss',
        greater_is_better=False,
        report_to='none',
        seed=SEED,
        data_seed=SEED,
        max_length=length,
        dataset_text_field=None if masked else 'text',
        packing=False,
    )
