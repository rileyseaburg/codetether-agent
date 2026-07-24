"""Typed schemas for Parquet-backed Iceberg cleanup tables."""

TOOL_CALL = """STRUCT<
  id: STRING, type: STRING,
  function: STRUCT<name: STRING, arguments: STRING>
>"""
MESSAGE = f"""STRUCT<
  role: STRING, content: STRING, name: STRING, tool_call_id: STRING,
  tool_calls: ARRAY<{TOOL_CALL}>
>"""
SAMPLES = f"""
  run_id STRING, sample_id STRING, cleanup_version INT,
  source_uri STRING, source_sha256 STRING, source_lines ARRAY<INT>,
  sender_id STRING, correlation_id STRING, quality_tier STRING,
  start_timestamp TIMESTAMP, end_timestamp TIMESTAMP,
  messages ARRAY<{MESSAGE}>, cleaned_at TIMESTAMP,
  source_records ARRAY<STRUCT<uri: STRING, sha256: STRING, line: INT>>,
  chunk_index INT, chunk_count INT, message_chars BIGINT
"""
QUARANTINE = """
  run_id STRING, quarantine_id STRING, cleanup_version INT,
  source_uri STRING, source_sha256 STRING, source_line INT,
  reason STRING, record_json STRING, cleaned_at TIMESTAMP
"""
MANIFESTS = """
  run_id STRING, cleanup_version INT, source_uri STRING, source_sha256 STRING,
  input_lines BIGINT, accepted_samples BIGINT, accepted_records BIGINT,
  quarantined_records BIGINT, quarantine_reasons MAP<STRING, BIGINT>,
  accepted_sha256 STRING, quarantine_sha256 STRING, cleaned_at TIMESTAMP
"""

TABLES = (
    ('samples', SAMPLES, 'days(start_timestamp), quality_tier'),
    ('quarantine', QUARANTINE, 'days(cleaned_at), reason'),
    ('manifests', MANIFESTS, 'days(cleaned_at)'),
)
