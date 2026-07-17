"""Stable-key Iceberg MERGE statements."""


def statement(table: str, view: str, key: str) -> str:
    """Build an insert-only MERGE that deduplicates across cleanup runs."""
    return (
        f'MERGE INTO {table} target USING {view} source '
        f'ON target.{key} = source.{key} '
        'WHEN NOT MATCHED THEN INSERT *'
    )
