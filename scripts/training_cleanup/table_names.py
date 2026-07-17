"""Qualified Iceberg table names used by the cleanup job."""

from dataclasses import dataclass


@dataclass(frozen=True)
class TableNames:
    """Names of the accepted, quarantine, and manifest tables."""

    samples: str
    quarantine: str
    manifests: str


def names(catalog: str, namespace: str, prefix: str) -> TableNames:
    """Build fully qualified table identifiers from validated components."""
    root = f'{catalog}.{namespace}.{prefix}'
    return TableNames(
        samples=f'{root}_samples',
        quarantine=f'{root}_quarantine',
        manifests=f'{root}_cleanup_manifests',
    )
