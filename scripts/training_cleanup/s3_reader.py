"""Executor-local reading and decoding of immutable S3 objects."""

from collections.abc import Iterable, Iterator
from typing import Protocol, cast

from .s3_client import create


class ObjectBody(Protocol):
    """Streaming response body contract returned by boto3."""

    def read(self) -> bytes:
        """Read all object bytes."""
        ...

    def close(self) -> None:
        """Release the response connection."""
        ...


def partition(
    keys: Iterable[str], endpoint: str, bucket: str
) -> Iterator[tuple[str, str]]:
    """Read one Spark partition through a reused S3 connection pool."""
    client = create(endpoint)
    for key in keys:
        response = client.get_object(Bucket=bucket, Key=key)
        body = cast(ObjectBody, response['Body'])
        try:
            text = body.read().decode('utf-8', errors='replace')
        finally:
            body.close()
        yield f's3a://{bucket}/{key}', text
