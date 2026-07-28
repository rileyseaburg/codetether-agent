"""Load the pinned base model for 8 GiB QLoRA training."""

import torch

from transformers import (
    AutoModelForCausalLM,
    AutoTokenizer,
    BitsAndBytesConfig,
)

from .attention import implementation
from .distributed import local_rank
from .model_target import resolve_target
from .precision import compute_dtype


def load() -> tuple[object, object]:
    """Return a 4-bit model and its exact-revision tokenizer."""
    if not torch.cuda.is_available():
        raise RuntimeError('CUDA GPU is required for the QLoRA training run')
    capability = torch.cuda.get_device_capability()
    if capability < (7, 0):
        raise RuntimeError(f'unsupported CUDA capability: {capability}')
    model_id, revision = resolve_target()
    dtype = compute_dtype()
    quantization = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type='nf4',
        bnb_4bit_use_double_quant=True,
        bnb_4bit_compute_dtype=dtype,
    )
    tokenizer = AutoTokenizer.from_pretrained(
        model_id,
        revision=revision,
        use_fast=True,
    )
    tokenizer.pad_token = tokenizer.eos_token
    tokenizer.padding_side = 'right'
    model = AutoModelForCausalLM.from_pretrained(
        model_id,
        revision=revision,
        quantization_config=quantization,
        dtype=dtype,
        device_map={'': local_rank()},
        attn_implementation=implementation(),
    )
    model.config.use_cache = False
    return model, tokenizer
