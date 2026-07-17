"""Concurrent discovery of day-sized S3 key-prefix shards."""

from concurrent.futures import ThreadPoolExecutor
from functools import partial
from typing import cast

from .s3_client import S3Client


def day_shards(client: S3Client, bucket: str, root: str) -> list[str]:
    """Discover hierarchical shards down to the source day level."""
    years = _children(client, bucket, root)
    months = _next_level(client, bucket, years or [root])
    days = _next_level(client, bucket, months or years or [root])
    return days or months or years or [root]


def _next_level(client: S3Client, bucket: str, parents: list[str]) -> list[str]:
    with ThreadPoolExecutor(max_workers=16) as pool:
        groups = pool.map(partial(_children, client, bucket), parents)
        return [prefix for group in groups for prefix in group]


def _children(client: S3Client, bucket: str, prefix: str) -> list[str]:
    response = client.list_objects_v2(
        Bucket=bucket, Prefix=prefix, Delimiter='/', MaxKeys=1000
    )
    entries = cast(list[dict[str, object]], response.get('CommonPrefixes', []))
    return [cast(str, entry['Prefix']) for entry in entries]
