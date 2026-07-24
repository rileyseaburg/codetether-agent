"""Command-line settings for governed dataset exports."""

import argparse

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Settings:
    """Bounded export parameters."""

    output: Path
    train_limit: int
    validation_limit: int


def parse() -> Settings:
    """Parse explicit export bounds and destination."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--train-limit', type=int, default=5000)
    parser.add_argument('--validation-limit', type=int, default=500)
    values = parser.parse_args()
    return Settings(values.output, values.train_limit, values.validation_limit)
