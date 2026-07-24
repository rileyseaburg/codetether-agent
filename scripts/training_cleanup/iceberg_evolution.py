"""Additive Iceberg schema evolution for trainer samples."""

_COLUMNS = {
    'source_records': 'ARRAY<STRUCT<uri: STRING, sha256: STRING, line: INT>>',
    'chunk_index': 'INT',
    'chunk_count': 'INT',
    'message_chars': 'BIGINT',
}


def samples(spark: object, table: str) -> None:
    """Add cross-object provenance and chunk fields to an existing table."""
    existing = set(spark.table(table).columns)
    for name, data_type in _COLUMNS.items():
        if name not in existing:
            spark.sql(f'ALTER TABLE {table} ADD COLUMNS ({name} {data_type})')
