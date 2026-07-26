"""Merge a trained LoRA adapter into the pinned base model."""

import argparse
import json

from pathlib import Path

import torch

from peft import PeftModel
from transformers import AutoModelForCausalLM, AutoTokenizer

from .constants import BASE_MODEL, BASE_REVISION
from .merge_guard import verify


def main() -> None:
    """Create a standalone local model from the adapter checkpoint."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--adapter', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--model', default=BASE_MODEL)
    values = parser.parse_args()
    budget = verify(values.model, values.output)
    base = AutoModelForCausalLM.from_pretrained(
        values.model,
        revision=BASE_REVISION,
        dtype=torch.bfloat16,
        device_map='cpu',
        low_cpu_mem_usage=True,
    )
    merged = PeftModel.from_pretrained(base, values.adapter).merge_and_unload()
    values.output.mkdir(parents=True, exist_ok=True)
    merged.save_pretrained(
        values.output,
        safe_serialization=True,
        max_shard_size='2GB',
    )
    tokenizer = AutoTokenizer.from_pretrained(
        values.model,
        revision=BASE_REVISION,
    )
    tokenizer.save_pretrained(values.output)
    evidence = {
        'base_model': BASE_MODEL,
        'base_revision': BASE_REVISION,
        'adapter': str(values.adapter),
        'output': str(values.output),
        'dtype': 'bfloat16',
        'budget': budget,
    }
    print(json.dumps(evidence, sort_keys=True))


if __name__ == '__main__':
    main()
