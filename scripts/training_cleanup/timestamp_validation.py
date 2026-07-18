"""Timezone-aware source timestamp validation."""

import re

from datetime import datetime


_NANOSECONDS = re.compile(r'(\.\d{6})\d+(?=Z|[+-]\d{2}:\d{2}$)')


def valid(value: str | None) -> bool:
    """Return whether a timestamp is ISO-8601 and timezone aware."""
    if not value:
        return False
    try:
        normalized = _NANOSECONDS.sub(r'\1', value)
        if normalized.endswith('Z'):
            normalized = f'{normalized[:-1]}+00:00'
        return datetime.fromisoformat(normalized).tzinfo is not None
    except ValueError:
        return False
