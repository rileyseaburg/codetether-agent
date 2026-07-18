"""Concurrent discovery of hour-sized S3 key-prefix shards."""

from concurrent.futures import ThreadPoolExecutor
from functools import partial
from typing import cast

from .s3_client import S3Client


def hour_shards(client: S3Client, bucket: str, root: str) -> list[str]:
    """Discover hierarchical shards down to the source hour level."""
    years = _children(client, bucket, root)
    months = _next_level(client, bucket, years or [root])
    days = _next_level(client, bucket, months or years or [root])
    hours = _next_level(client, bucket, days or months or years or [root])
    return hours or days or months or years or [root]


def _next_level(client: S3Client, bucket: str, parents: list[str]) -> list[str]:
    with ThreadPoolExecutor(max_workers=8) as pool:
        groups = pool.map(partial(_children, client, bucket), parents)
        return [prefix for group in groups for prefix in group]


def _children(client: S3Client, bucket: str, prefix: str) -> list[str]:
    response = client.list_objects_v2(
        Bucket=bucket, Prefix=prefix, Delimiter='/', MaxKeys=1000
    )
    entries = cast(list[dict[str, object]], response.get('CommonPrefixes', []))
    return [cast(str, entry['Prefix']) for entry in entries]
