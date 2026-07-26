"""Estimate QLoRA memory so a run fails fast instead of mid-epoch.

A mixture-of-experts model keeps every expert resident under QLoRA, so
total parameters decide feasibility even when only a few are active.

Two measured runs shaped these constants:

- A100 40 GB: reached 39.47 GiB and hit CUDA OOM while loading weights,
  against a naive prediction of 16.8 GiB.
- A100 80 GB: weights loaded, then `prepare_model_for_kbit_training`
  upcast non-quantized parameters to float32 and reached 78.38 GiB.

The model stores 18,432 expert tensors, so NF4 block scales and per-layer
routers cost far more than a flat bytes-per-parameter rule implies.
"""

BYTES_PER_4BIT_PARAM = 0.55
"""NF4 weight plus double-quantization and per-block scale overhead."""

MOE_TENSOR_OVERHEAD = 2.4
"""Multiplier for models sharded into thousands of small expert tensors.

Derived from the observed 39.47 GiB load against a 16.8 GiB flat estimate.
"""

ADAPTER_OVERHEAD_GB = 1.5
ACTIVATION_GB_PER_1K = 0.45
"""Approximate activation cost per 1k tokens with gradient checkpointing."""


def estimate(
    parameters_billion: float, sequence: int, mixture_of_experts: bool = False
) -> dict[str, float]:
    """Return estimated gigabytes for weights, activations, and total."""
    weights = parameters_billion * BYTES_PER_4BIT_PARAM
    if mixture_of_experts:
        weights *= MOE_TENSOR_OVERHEAD
    activations = (sequence / 1000.0) * ACTIVATION_GB_PER_1K
    total = weights + activations + ADAPTER_OVERHEAD_GB
    return {
        'weights_gb': round(weights, 1),
        'activations_gb': round(activations, 1),
        'total_gb': round(total, 1),
    }


def fits(
    parameters_billion: float,
    sequence: int,
    available_gb: float,
    mixture_of_experts: bool = False,
) -> bool:
    """Return whether the configuration fits with a safety margin."""
    total = estimate(parameters_billion, sequence, mixture_of_experts)
    return total['total_gb'] <= available_gb * 0.9
