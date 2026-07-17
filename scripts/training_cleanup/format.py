"""Stable accepted, quarantine, and manifest JSON serialization."""

from collections import Counter

from .model import CleanResult, JsonObject
from .serialization import digest, json_line


def accepted_lines(result: CleanResult) -> list[str]:
    """Serialize accepted conversation samples."""
    return [json_line(sample) for sample in result.samples]


def quarantine_lines(result: CleanResult, source_uri: str) -> list[str]:
    """Serialize every rejected source value with its reason."""
    return [
        json_line(
            {
                'source_uri': source_uri,
                'source_line': record.line,
                'reason': record.reason,
                'record': record.value,
            }
        )
        for record in result.rejected
    ]


def manifest(
    result: CleanResult, source_uri: str, source_text: str
) -> JsonObject:
    """Build an auditable per-object cleanup manifest."""
    accepted = accepted_lines(result)
    quarantine = quarantine_lines(result, source_uri)
    reasons = Counter(record.reason for record in result.rejected)
    return {
        'cleanup_version': 1,
        'source_uri': source_uri,
        'source_sha256': digest(source_text),
        'input_lines': result.input_lines,
        'accepted_samples': len(result.samples),
        'accepted_records': result.accepted_records,
        'quarantined_records': len(result.rejected),
        'quarantine_reasons': dict(sorted(reasons.items())),
        'accepted_sha256': digest('\n'.join(accepted)),
        'quarantine_sha256': digest('\n'.join(quarantine)),
    }
