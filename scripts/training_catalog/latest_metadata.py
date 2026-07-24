"""Iceberg metadata discovery for catalog recovery."""

from typing import cast

from training_cleanup.s3_client import create


def find(endpoint: str, bucket: str, table: str) -> str:
    """Return the newest immutable metadata JSON location for a table."""
    client = create(endpoint)
    prefix = f'iceberg/codetether/{table}/metadata/'
    paginator = client.get_paginator('list_objects_v2')
    objects: list[dict[str, object]] = []
    for page in paginator.paginate(Bucket=bucket, Prefix=prefix):
        objects.extend(
            item
            for item in cast(list[dict[str, object]], page.get('Contents', []))
            if cast(str, item['Key']).endswith('.metadata.json')
        )
    if not objects:
        raise RuntimeError(f'no Iceberg metadata found for {table}')
    latest = max(objects, key=lambda item: item['LastModified'])
    return f's3://{bucket}/{latest["Key"]}'
