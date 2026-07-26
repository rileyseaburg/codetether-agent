"""Command-line settings for one constrained GPU training run."""

import argparse

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Settings:
    """Input data and checkpoint destinations."""

    train: Path
    validation: Path
    output: Path
    epochs: float
    resume: Path | None
    adapter: Path | None
    masked: bool


def parse() -> Settings:
    """Parse explicit training inputs and bounds."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--train', type=Path, required=True)
    parser.add_argument('--validation', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--epochs', type=float, default=1.0)
    parser.add_argument('--resume', type=Path)
    parser.add_argument('--adapter', type=Path)
    parser.add_argument('--masked', action='store_true')
    values = parser.parse_args()
    return Settings(
        values.train,
        values.validation,
        values.output,
        values.epochs,
        values.resume,
        values.adapter,
        values.masked,
    )
