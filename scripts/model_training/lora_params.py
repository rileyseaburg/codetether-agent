"""Adapter topology for parameter-efficient fine-tuning."""

LORA_RANK = 16
LORA_ALPHA = 32
LORA_DROPOUT = 0.05

TARGET_MODULES = [
    'q_proj',
    'k_proj',
    'v_proj',
    'o_proj',
]
"""Attention projections only.

Mixture-of-experts models repeat `gate_proj`, `up_proj`, and `down_proj`
once per expert, so targeting them multiplied trainable parameters by the
expert count and exhausted an 80 GB device: roughly 1.2 B trainable
parameters instead of 12.6 M.
"""
