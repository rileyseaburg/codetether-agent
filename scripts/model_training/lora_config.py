"""Parameter-efficient adapter configuration."""

from peft import LoraConfig

from .constants import (
    LORA_ALPHA,
    LORA_DROPOUT,
    LORA_RANK,
    TARGET_MODULES,
)


def build() -> LoraConfig:
    """Return the adapter topology for all Qwen projection layers."""
    return LoraConfig(
        r=LORA_RANK,
        lora_alpha=LORA_ALPHA,
        lora_dropout=LORA_DROPOUT,
        target_modules=TARGET_MODULES,
        bias='none',
        task_type='CAUSAL_LM',
    )
