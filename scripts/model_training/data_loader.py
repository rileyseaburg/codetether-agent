"""Hugging Face dataset loading for rendered training files."""

from pathlib import Path

from datasets import Dataset, load_dataset


SUPERVISION_COLUMNS = ('prompt', 'completion')
"""TRL infers prompt-completion mode from the column names alone.

Any extra column, such as provenance metadata, breaks collation, so it is
dropped after loading while the manifests retain full provenance.
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
    return _project(train_data), _project(validation_data)


def _project(data: Dataset) -> Dataset:
    """Keep only the columns TRL expects for supervised training."""
    if 'prompt' not in data.column_names:
        return data
    extra = [c for c in data.column_names if c not in SUPERVISION_COLUMNS]
    return data.remove_columns(extra) if extra else data
