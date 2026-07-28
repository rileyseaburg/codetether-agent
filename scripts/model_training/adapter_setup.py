"""Prepare a new or existing adapter for quantized training."""

from pathlib import Path

from peft import PeftModel

from .adapter_dtype import align
from .kbit_prepare import prepare
from .lora_config import build


def configure(
    model: object, adapter: Path | None
) -> tuple[object, object | None]:
    """Prepare k-bit layers and optionally attach a trainable adapter."""
    prepared = prepare(model)
    if adapter is None:
        return prepared, build()
    attached = PeftModel.from_pretrained(
        prepared,
        adapter,
        is_trainable=True,
    )
    return align(attached), None
