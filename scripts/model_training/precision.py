"""Select training precision from device capability.

Ampere and newer support bfloat16, which avoids the loss-scaling
instability of float16 and runs faster on H100-class hardware.
"""

import torch


BF16_CAPABILITY = (8, 0)


def supports_bf16() -> bool:
    """Return whether the current device should train in bfloat16."""
    if not torch.cuda.is_available():
        return False
    return torch.cuda.get_device_capability() >= BF16_CAPABILITY


def compute_dtype() -> torch.dtype:
    """Return the dtype to use for quantized compute and training."""
    return torch.bfloat16 if supports_bf16() else torch.float16
