"""Timezone-aware source timestamp validation."""

from datetime import datetime


def valid(value: str | None) -> bool:
    """Return whether a timestamp is ISO-8601 and timezone aware."""
    if not value:
        return False
    try:
        return (
            datetime.fromisoformat(value.replace('Z', '+00:00')).tzinfo
            is not None
        )
    except ValueError:
        return False
