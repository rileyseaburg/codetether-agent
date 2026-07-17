"""Environment-backed Polaris provisioning settings."""

import os

from dataclasses import dataclass


@dataclass(frozen=True)
class Settings:
    """Credentials, endpoints, and resource names for provisioning."""

    uri: str
    realm: str
    root_id: str
    root_secret: str
    pipeline_id: str
    pipeline_secret: str
    storage_endpoint: str
    storage_location: str


def load() -> Settings:
    """Load every required setting or fail without printing a secret."""
    return Settings(
        uri=_required('POLARIS_URI').rstrip('/'),
        realm=os.getenv('POLARIS_REALM', 'POLARIS'),
        root_id=_required('POLARIS_ROOT_ID'),
        root_secret=_required('POLARIS_ROOT_SECRET'),
        pipeline_id=_required('POLARIS_PIPELINE_ID'),
        pipeline_secret=_required('POLARIS_PIPELINE_SECRET'),
        storage_endpoint=_required('MINIO_ENDPOINT'),
        storage_location=_required('POLARIS_STORAGE_LOCATION'),
    )


def _required(name: str) -> str:
    value = os.getenv(name)
    if not value:
        raise RuntimeError(f'{name} is required')
    return value
