"""Detect the fastest available attention implementation.

FlashAttention-2 reduces memory and time, and is required for padding-free
batching. It is an optional dependency, so absence must degrade cleanly
rather than fail the run.
"""

import importlib.util

import torch


MIN_CAPABILITY = (8, 0)
"""FlashAttention-2 requires Ampere or newer."""


def flash_available() -> bool:
    """Return whether FlashAttention-2 can be used on this device."""
    if importlib.util.find_spec('flash_attn') is None:
        return False
    if not torch.cuda.is_available():
        return False
    return torch.cuda.get_device_capability() >= MIN_CAPABILITY


def implementation() -> str:
    """Return the attention implementation to request from transformers."""
    return 'flash_attention_2' if flash_available() else 'sdpa'
