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
            'reprocess': settings.reprocess,
            'source': settings.source_uri,
            'source_before': settings.cutoff_uri,
            'tables': settings.tables.__dict__,
            'accepted_samples': counts.get('samples', 0),
            'quality_tiers': _details(counts, 'samples:'),
            'quarantined_records': counts.get('quarantine', 0),
            'quarantine_reasons': _details(counts, 'quarantine:'),
            'source_objects': counts.get('manifests', 0),
        },
        sort_keys=True,
    )


def _details(counts: Mapping[object, int], prefix: str) -> dict[str, int]:
    return {
        str(key).removeprefix(prefix): value
        for key, value in counts.items()
        if str(key).startswith(prefix)
    }
