"""Fail-closed validation for exported conversation splits."""

import json

from pathlib import Path


def splits(train: Path, validation: Path) -> dict[str, object]:
    """Verify JSON shape, role coverage, and correlation isolation."""
    train_ids, train_roles = _scan(train)
    validation_ids, validation_roles = _scan(validation)
    overlap = train_ids & validation_ids
    if overlap:
        raise ValueError(f'correlation leakage: {len(overlap)} identifiers')
    required = {'user', 'assistant'}
    if not required <= train_roles or not required <= validation_roles:
        raise ValueError('both splits must contain user and assistant roles')
    return {
        'correlation_overlap': 0,
        'train_roles': sorted(train_roles),
        'validation_roles': sorted(validation_roles),
    }


def _scan(path: Path) -> tuple[set[str], set[str]]:
    correlations: set[str] = set()
    roles: set[str] = set()
    for number, raw in enumerate(path.read_text().splitlines(), 1):
        value = json.loads(raw)
        messages = value.get('messages')
        metadata = value.get('metadata')
        if not isinstance(messages, list) or not isinstance(metadata, dict):
            raise ValueError(f'invalid record at {path}:{number}')
        correlations.add(str(metadata['correlation_id']))
        roles.update(
            str(item.get('role')) for item in messages if isinstance(item, dict)
        )
    return correlations, roles
