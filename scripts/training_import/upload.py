"""Immutable S3 upload operations for VM session objects."""

import hashlib

from typing import Protocol


class Client(Protocol):
    """Minimal object-store client used by immutable uploads."""

    def head_object(self, **kwargs: object) -> dict[str, object]: ...
    def put_object(self, **kwargs: object) -> dict[str, object]: ...


def one(client: Client, bucket: str, key: str, payload: bytes) -> str:
    """Upload a missing object or verify its immutable digest."""
    checksum = hashlib.sha256(payload).hexdigest()
    try:
        response = client.head_object(Bucket=bucket, Key=key)
        metadata = response.get('Metadata', {})
        if isinstance(metadata, dict) and metadata.get('sha256') == checksum:
            return 'existing'
        raise RuntimeError(f'immutable key conflict: {key}')
    except Exception as error:
        code = getattr(error, 'response', {}).get('Error', {}).get('Code')
        if code not in {'404', 'NoSuchKey', 'NotFound'}:
            raise
    client.put_object(
        Bucket=bucket,
        Key=key,
        Body=payload,
        ContentType='application/x-ndjson',
        Metadata={'sha256': checksum, 'source': 'vm-session-import'},
    )
    return 'uploaded'
