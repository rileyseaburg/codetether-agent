"""Hugging Face dataset loading for rendered training files."""

from pathlib import Path

from datasets import Dataset, load_dataset

from .constants import MAX_EVAL_SAMPLES, SEED


SUPERVISION_COLUMNS = ('prompt', 'completion')
"""TRL infers prompt-completion mode from the column names alone.

Any extra column, such as provenance metadata, breaks collation, so it is
dropped after loading while the manifests retain full provenance.

A `length` column was added here to enable `group_by_length`, but retaining
it broke the run before the first optimizer step: run 8 reached the training
stage and exited non-zero, where the identical configuration without the
column had trained to step 267 with loss falling from 2.5903 to 1.6829.
"""


def splits(train: Path, validation: Path) -> tuple[Dataset, Dataset]:
    """Load immutable JSONL paths as independent datasets."""
    train_data = load_dataset(
        'json',
        data_files=str(train),
        split='train',
    )
    validation_data = load_dataset(
        'json',
        data_files=str(validation),
        split='train',
    )
    return _project(train_data), _project(_subset(validation_data))


def _subset(data: Dataset) -> Dataset:
    """Bound the validation split so evaluation stays affordable."""
    if len(data) <= MAX_EVAL_SAMPLES:
        return data
    shuffled = data.shuffle(seed=SEED)
    return shuffled.select(range(MAX_EVAL_SAMPLES))


def _project(data: Dataset) -> Dataset:
    """Keep only the columns TRL expects for supervised training."""
    if 'prompt' not in data.column_names:
        return data
    extra = [c for c in data.column_names if c not in SUPERVISION_COLUMNS]
    return data.remove_columns(extra) if extra else data
