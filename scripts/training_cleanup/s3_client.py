"""Configured boto3 client construction for MinIO access."""

from typing import Protocol, cast


class S3Client(Protocol):
    """Minimal client contract used by discovery and object readers."""

    def get_paginator(self, name: str) -> object:
        """Return an AWS API paginator."""
        ...

    def list_objects_v2(self, **kwargs: object) -> dict[str, object]:
        """List keys or common prefixes."""
        ...

    def get_object(self, **kwargs: object) -> dict[str, object]:
        """Open one object body."""
        ...


def options() -> dict[str, object]:
    """Return resilient connection policy for historical MinIO scans."""
    return {
        'retries': {'max_attempts': 10, 'mode': 'adaptive'},
        'connect_timeout': 10,
        'read_timeout': 120,
        'max_pool_connections': 16,
        's3': {'addressing_style': 'path'},
    }


def create(endpoint: str) -> S3Client:
    """Create a path-style, bounded-retry S3 client."""
    import boto3

    from botocore.config import Config

    config = Config(**options())
    return cast(
        S3Client, boto3.client('s3', endpoint_url=endpoint, config=config)
    )
