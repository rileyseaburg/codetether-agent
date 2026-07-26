"""Message-digest shard predicate for memory-bounded exports."""

BUCKETS = list('0123456789abcdef')


def shard(bucket: str | None) -> str:
    """Restrict a scan to one leading message-digest hex digit."""
    if bucket is None:
        return ''
    return (
        'AND substr(to_hex(sha256(to_utf8('
        'json_format(CAST(messages AS JSON))))), 1, 1) '
        f"= '{bucket}'"
    )
