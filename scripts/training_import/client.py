"""Configured MinIO client for historical session imports."""

import os

import boto3

from botocore.config import Config


def create() -> object:
    """Create a path-style client with adaptive historical-read retries."""
    config = Config(
        retries={'max_attempts': 12, 'mode': 'adaptive'},
        read_timeout=120,
        max_pool_connections=16,
        s3={'addressing_style': 'path'},
    )
    return boto3.client(
        's3', endpoint_url=os.environ['MINIO_ENDPOINT'], config=config
    )
