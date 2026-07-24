"""Deterministic timestamps for snapshots without per-message time."""

from datetime import datetime, timedelta, timezone


def timestamp(value: object, index: int) -> str:
    """Return the session creation second plus a stable microsecond offset."""
    raw = str(value or '1970-01-01T00:00:00')[:19]
    try:
        start = datetime.strptime(raw, '%Y-%m-%dT%H:%M:%S').replace(
            tzinfo=timezone.utc
        )
    except ValueError:
        start = datetime(1970, 1, 1, tzinfo=timezone.utc)
    return (
        (start + timedelta(microseconds=index))
        .isoformat()
        .replace('+00:00', 'Z')
    )
