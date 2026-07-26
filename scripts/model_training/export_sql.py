"""Deduplicated candidate SQL body shared by governed exports."""

from .export_bounds import MAX_CHARS, MIN_CHARS


def body(snapshot_id: int, shard: str, filtered: str) -> str:
    """Return the candidate and dedupe CTEs for one scan."""
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
      AND message_chars BETWEEN {MIN_CHARS} AND {MAX_CHARS}
      {shard}
      {filtered}
), deduplicated AS (
    SELECT *, row_number() OVER (
        PARTITION BY message_sha ORDER BY sample_id
    ) AS duplicate_rank
    FROM candidates
)
""".strip()
