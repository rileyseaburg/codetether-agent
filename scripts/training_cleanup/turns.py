"""Split a correlation group into individual user turns."""

from .model import SourceRecord
from .record import is_user, text


def split(
    records: list[SourceRecord],
) -> tuple[list[SourceRecord], list[list[SourceRecord]]]:
    """Return pre-prompt records and turns beginning with one user prompt."""
    prefix: list[SourceRecord] = []
    turns: list[list[SourceRecord]] = []
    current: list[SourceRecord] | None = None
    for record in records:
        if is_user(record):
            if current:
                turns.append(current)
            system = [item for item in prefix if text(item, 'role') == 'system']
            prefix = [item for item in prefix if text(item, 'role') != 'system']
            current = [*system, record]
        elif current is None:
            prefix.append(record)
        else:
            current.append(record)
    if current:
        turns.append(current)
    return prefix, turns
