"""Per-object manifests for globally reconstructed conversations."""

from collections import Counter
from collections.abc import Iterable

from .manifest_hash import quarantine as quarantine_hash
from .outcome_events import Outcome
from .prepared_model import PreparedObject
from .provenance import identity
from .serialization import digest


def build(
    prepared: PreparedObject,
    outcomes: Iterable[Outcome],
    run_id: str,
    cleaned_at: str,
) -> dict[str, object]:
    """Account for every raw line after cross-object reconstruction."""
    accepted: set[str] = set()
    samples: set[str] = set()
    rejected: dict[str, str] = {}
    for kind, record_id, detail in outcomes:
        if kind == 'accepted':
            accepted.add(record_id)
        elif kind == 'sample':
            samples.add(record_id)
        elif kind == 'rejected':
            rejected[record_id] = detail
    expected = {
        identity(record) for record in [*prepared.records, *prepared.rejected]
    }
    resolved = accepted | rejected.keys()
    if accepted & rejected.keys() or resolved != expected:
        raise ValueError(
            f'incomplete outcome accounting: {prepared.source_uri}'
        )
    reasons = Counter(rejected.values())
    return {
        'run_id': run_id,
        'cleanup_version': 2,
        'source_uri': prepared.source_uri,
        'source_sha256': prepared.source_sha256,
        'input_lines': prepared.input_lines,
        'accepted_samples': len(samples),
        'accepted_records': len(accepted),
        'quarantined_records': len(rejected),
        'quarantine_reasons': dict(sorted(reasons.items())),
        'accepted_sha256': digest('\n'.join(sorted(accepted | samples))),
        'quarantine_sha256': quarantine_hash(rejected),
        'cleaned_at': cleaned_at,
    }
