"""Prepare a quantized model for training without a float32 upcast.

`peft.prepare_model_for_kbit_training` unconditionally casts every
non-quantized float16/bfloat16 parameter to float32, with no option to
disable it. Qwen3-Coder-30B-A3B holds 18,432 expert tensors plus per-layer
routers and norms, and that upcast drove an 80 GB A100 to 78.38 GiB before
CUDA OOM. bfloat16 training is numerically adequate and fits comfortably.

Skipping the upcast entirely is not sufficient. Newly created LoRA weights
default to float32 while the frozen base stays bfloat16, and the mismatch
raises `RuntimeError: expected scalar type BFloat16 but found Float` at the
first backward pass. Trainable parameters are therefore cast to the base
dtype explicitly, which aligns dtypes without duplicating every frozen
expert tensor in float32.
"""

import torch


def prepare(model: object) -> object:
    """Freeze base weights and enable gradient-friendly checkpointing."""
    for parameter in model.parameters():
        parameter.requires_grad = False
    _enable_input_grads(model)
    model.gradient_checkpointing_enable(
        gradient_checkpointing_kwargs={'use_reentrant': False}
    )
    model.config.use_cache = False
    return model


def _enable_input_grads(model: object) -> None:
    """Ensure checkpointed activations receive gradients."""
    if hasattr(model, 'enable_input_require_grads'):
        model.enable_input_require_grads()
        return

    def hook(_module: object, _args: object, output: torch.Tensor) -> None:
        output.requires_grad_(True)

    model.get_input_embeddings().register_forward_hook(hook)
