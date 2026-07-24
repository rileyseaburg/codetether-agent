"""Atomic message units that keep tool calls beside their responses."""

from .model import SourceRecord
from .record import source_order, text
from .tool_ids import call_ids


def build(records: list[SourceRecord]) -> list[list[SourceRecord]]:
    """Group validated tool calls and results into indivisible units."""
    units: list[list[SourceRecord]] = []
    owners: dict[str, int] = {}
    for record in records:
        identifiers = call_ids(record)
        response = text(record, 'tool_call_id')
        if identifiers:
            owners.update(
                (identifier, len(units)) for identifier in identifiers
            )
            units.append([record])
        elif response in owners:
            units[owners[response]].append(record)
        else:
            units.append([record])
    for unit in units:
        unit.sort(key=source_order)
    return sorted(units, key=lambda unit: source_order(unit[0]))
