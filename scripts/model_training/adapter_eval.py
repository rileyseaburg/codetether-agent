"""Load a base model and optional adapter for inference-only evaluation.

Separate from the training loader because evaluation needs a cache-enabled
model in eval mode and must tolerate an absent adapter to measure an
untuned baseline.
"""

import json

from pathlib import Path

import torch

from peft import PeftModel
from transformers import (
    AutoModelForCausalLM,
    AutoTokenizer,
    BitsAndBytesConfig,
)


def load_for_eval(
    adapter: Path | None, base: str | None = None
) -> tuple[object, object]:
    """Return a 4-bit model in eval mode and its tokenizer."""
    model_id = base or _base_from(adapter)
    quantization = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type='nf4',
        bnb_4bit_use_double_quant=True,
        bnb_4bit_compute_dtype=torch.float16,
    )
    source = str(adapter) if adapter else model_id
    tokenizer = AutoTokenizer.from_pretrained(source, use_fast=True)
    tokenizer.pad_token = tokenizer.pad_token or tokenizer.eos_token
    model = AutoModelForCausalLM.from_pretrained(
        model_id,
        quantization_config=quantization,
        dtype=torch.float16,
        device_map={'': 0},
    )
    if adapter:
        model = PeftModel.from_pretrained(model, str(adapter))
    model.eval()
    model.config.use_cache = True
    return model, tokenizer


def _base_from(adapter: Path | None) -> str:
    """Read the base model recorded in an adapter configuration."""
    if adapter is None:
        raise SystemExit('either --base or --adapter is required')
    config = json.loads((adapter / 'adapter_config.json').read_text())
    return str(config['base_model_name_or_path'])
