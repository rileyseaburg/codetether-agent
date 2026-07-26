"""Snapshot-pinned SQL for correlation-isolated model exports."""

from .export_bounds import redaction
from .export_shard import shard as _shard
from .export_sql import body


def build(
    split: str,
    limit: int,
    snapshot_id: int,
    encrypted: bool = True,
    bucket: str | None = None,
) -> str:
    """Return deterministic Trino SQL for one dataset split.

    `bucket` restricts the scan to one leading hex digit of the message
    digest so the deduplicating window stays inside Trino's per-node
    memory limit at full-corpus scale.
    """
    predicate = (
        "split_bucket = '0'" if split == 'validation' else "split_bucket <> '0'"
    )
    filtered = redaction(encrypted)
    shard = _shard(bucket)
    return f"""
{body(snapshot_id, shard, filtered)}
SELECT sample_id, correlation_id, message_sha, messages_json
FROM deduplicated
WHERE duplicate_rank = 1 AND {predicate}
ORDER BY message_sha, sample_id
LIMIT {limit}
""".strip()


def manifest(snapshot_id: int) -> str:
    """Return the source snapshot metadata query."""
    return f"""
SELECT snapshot_id, parent_id, committed_at, operation, manifest_list
FROM \"training_samples$snapshots\"
WHERE snapshot_id = {snapshot_id}
""".strip()


def latest_snapshot() -> str:
    """Return the newest governed table snapshot query."""
    return """
SELECT snapshot_id
FROM \"training_samples$snapshots\"
ORDER BY committed_at DESC
LIMIT 1
""".strip()
