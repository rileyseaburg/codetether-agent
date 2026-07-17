"""Typed TOML benchmark-manifest loading."""

from pathlib import Path
from typing import cast
import tomllib

from .case import BenchmarkCase


def load(path: Path) -> list[BenchmarkCase]:
    """Load and validate all cases in a manifest."""
    with path.open("rb") as handle:
        document = tomllib.load(handle)
    raw_cases = document.get("case")
    if not isinstance(raw_cases, list) or not raw_cases:
        raise ValueError("manifest must contain at least one [[case]]")
    return [_parse(path.parent, item) for item in raw_cases]


def _parse(base: Path, item: object) -> BenchmarkCase:
    if not isinstance(item, dict):
        raise ValueError("each case must be a TOML table")
    values = cast(dict[str, object], item)
    timeout = values.get("timeout_seconds", 300)
    if not isinstance(timeout, int) or timeout < 1:
        raise ValueError("timeout_seconds must be a positive integer")
    identifier = _text(values, "id")
    fixture = (base / _text(values, "fixture")).resolve()
    if not fixture.is_dir():
        raise ValueError(f"fixture directory does not exist: {fixture}")
    return BenchmarkCase(
        identifier=identifier,
        fixture=fixture,
        prompt=_text(values, "prompt"),
        checks=_texts(values, "checks"),
        protected=_texts(values, "protected"),
        timeout_seconds=timeout,
    )


def _text(values: dict[str, object], key: str) -> str:
    value = values.get(key)
    if not isinstance(value, str) or not value.strip():
        raise ValueError(f"{key} must be a non-empty string")
    return value


def _texts(values: dict[str, object], key: str) -> tuple[str, ...]:
    value = values.get(key)
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        raise ValueError(f"{key} must be an array of strings")
    return tuple(cast(list[str], value))
