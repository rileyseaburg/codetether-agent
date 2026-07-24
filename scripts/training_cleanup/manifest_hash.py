"""Stable hashes for per-object cleanup outcomes."""

from .serialization import digest


def quarantine(rejected: dict[str, str]) -> str:
    """Hash rejected record identities together with their reasons."""
    values = (
        f'{record_id}:{reason}'
        for record_id, reason in sorted(rejected.items())
    )
    return digest('\n'.join(values))
