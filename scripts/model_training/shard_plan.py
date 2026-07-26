"""Bundled parameters for one sharded split export."""

from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path


Runner = Callable[[str], list[dict[str, object]]]


@dataclass(frozen=True)
class ShardPlan:
    """Immutable inputs describing how to export one split."""

    output: Path
    buckets: list[str]
    per_shard: int
    snapshot: int
    runner: Runner
