"""Distributed manifest accounting for global cleanup."""

from .global_rows import manifest
from .manifest_global import build
from .outcome_events import rejection
from .outcome_events import sample as sample_events
from .settings import Settings


def rows(
    prepared: object,
    samples: object,
    rejected: object,
    settings: Settings,
    timestamp: str,
) -> object:
    """Build one balanced manifest row per immutable source object."""
    outcomes = samples.flatMap(
        lambda value: sample_events(value, settings.run_id, timestamp)
    ).union(rejected.map(rejection))
    joined = prepared.map(
        lambda value: (value.source_uri, value)
    ).leftOuterJoin(outcomes.groupByKey())
    return joined.map(
        lambda item: manifest(
            build(item[1][0], item[1][1] or (), settings.run_id, timestamp)
        )
    )
