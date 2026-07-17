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


def create(endpoint: str) -> S3Client:
    """Create a path-style, bounded-retry S3 client."""
    import boto3

    from botocore.config import Config

    config = Config(
        retries={'max_attempts': 5, 'mode': 'standard'},
        max_pool_connections=64,
        s3={'addressing_style': 'path'},
    )
    return cast(
        S3Client, boto3.client('s3', endpoint_url=endpoint, config=config)
    )
