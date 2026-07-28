"""Align trainable adapter weights with the frozen base dtype.

LoRA weights are created in float32 while a quantized base model keeps its
bfloat16 compute dtype. The mismatch raises
`RuntimeError: expected scalar type BFloat16 but found Float` at the first
backward pass, which surfaced only after a GPU had trained for minutes.
"""

import torch


def base_dtype(model: object) -> torch.dtype:
    """Return the dtype of the frozen base weights."""
    for parameter in model.parameters():
        if not parameter.requires_grad and parameter.dtype in _HALF:
            return parameter.dtype
    return torch.bfloat16


_HALF = (torch.bfloat16, torch.float16)


def align(model: object) -> object:
    """Cast every trainable parameter to the frozen base dtype."""
    target = base_dtype(model)
    for parameter in model.parameters():
        if parameter.requires_grad and parameter.dtype not in _HALF:
            parameter.data = parameter.data.to(target)
    return model
