"""Aggregate metric extraction from tagged output rows."""

import json

from collections.abc import Iterator
from typing import cast

from .model import JsonObject
from .spark_transform import TaggedLine


def pairs(item: TaggedLine) -> Iterator[tuple[str, int]]:
    """Yield total and detail counters for one transformed output row."""
    kind, line = item
    yield kind, 1
    if kind == 'samples':
        yield f'samples:{_field(line, "quality_tier")}', 1
    elif kind == 'quarantine':
        yield f'quarantine:{_field(line, "reason")}', 1


def _field(line: str, name: str) -> str:
    value = cast(JsonObject, json.loads(line)).get(name)
    return value if isinstance(value, str) else 'unknown'
