"""Stable compact serialization and hashing helpers."""

import json

from hashlib import sha256


def digest(value: str) -> str:
    """Return a stable SHA-256 digest for serialized cleanup data."""
    return sha256(value.encode()).hexdigest()


def json_line(value: object) -> str:
    """Serialize one stable compact JSON value."""
    return json.dumps(value, sort_keys=True, separators=(',', ':'))
