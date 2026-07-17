"""Parallel paginated JSONL key listing over bounded S3 shards."""

from concurrent.futures import ThreadPoolExecutor
from functools import partial
from typing import Protocol, cast

from .s3_client import S3Client, create
from .s3_prefixes import day_shards
from .settings import Settings


class Paginator(Protocol):
    """Minimal boto3 paginator contract."""

    def paginate(self, **kwargs: object) -> object:
        """Yield response pages."""
        ...


def keys(settings: Settings) -> list[str]:
    """List immutable JSONL keys before the exclusive cutoff."""
    client = create(settings.endpoint)
    root = f'{settings.source_prefix.strip("/")}/'
    shards = day_shards(client, settings.bucket, root)
    with ThreadPoolExecutor(max_workers=32) as pool:
        groups = pool.map(
            partial(
                _shard_keys, client, settings.bucket, settings.source_before
            ),
            shards,
        )
        return sorted({key for group in groups for key in group})


def _shard_keys(
    client: S3Client, bucket: str, cutoff: str, prefix: str
) -> list[str]:
    paginator = cast(Paginator, client.get_paginator('list_objects_v2'))
    pages = cast(
        list[dict[str, object]],
        paginator.paginate(Bucket=bucket, Prefix=prefix),
    )
    entries = (
        cast(list[dict[str, object]], page.get('Contents', []))
        for page in pages
    )
    return [
        key
        for page_entries in entries
        for item in page_entries
        if (key := cast(str, item['Key'])).endswith('.jsonl') and key < cutoff
    ]
