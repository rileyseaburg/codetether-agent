"""Shared values for typed Iceberg cleanup rows."""

from datetime import datetime, timezone
from hashlib import sha256


def digest(value: str) -> str:
    """Return a stable SHA-256 identifier."""
    return sha256(value.encode()).hexdigest()


def cleaned_at() -> str:
    """Return a Spark-parseable UTC cleanup timestamp."""
    return datetime.now(timezone.utc).isoformat()
