"""Machine-readable cleanup run summaries."""

import json

from collections.abc import Mapping

from .settings import Settings


def build(settings: Settings, counts: Mapping[object, int]) -> str:
    """Serialize aggregate audit or apply counts and destination tables."""
    return json.dumps(
        {
            'mode': 'apply' if settings.apply else 'dry-run',
            'run_id': settings.run_id,
            'source': settings.source_uri,
            'source_before': settings.cutoff_uri,
            'tables': settings.tables.__dict__,
            'accepted_samples': counts.get('samples', 0),
            'quarantined_records': counts.get('quarantine', 0),
            'source_objects': counts.get('manifests', 0),
        },
        sort_keys=True,
    )
