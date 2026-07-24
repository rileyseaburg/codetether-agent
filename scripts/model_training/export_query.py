"""Snapshot-pinned SQL for correlation-isolated model exports."""


def build(split: str, limit: int, snapshot_id: int) -> str:
    """Return deterministic Trino SQL for one dataset split."""
    predicate = (
        "split_bucket = '0'" if split == 'validation' else "split_bucket <> '0'"
    )
    return f"""
WITH candidates AS (
    SELECT sample_id, correlation_id,
           json_format(CAST(messages AS JSON)) AS messages_json,
           to_hex(sha256(
               to_utf8(json_format(CAST(messages AS JSON)))
           )) AS message_sha,
           substr(to_hex(sha256(to_utf8(correlation_id))), 1, 1) AS split_bucket
    FROM training_samples FOR VERSION AS OF {snapshot_id}
    WHERE cleanup_version = 2
      AND message_chars BETWEEN 64 AND 8192
      AND json_format(CAST(messages AS JSON)) NOT LIKE '%encrypted_content%'
), deduplicated AS (
    SELECT *, row_number() OVER (
        PARTITION BY message_sha ORDER BY sample_id
    ) AS duplicate_rank
    FROM candidates
)
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
