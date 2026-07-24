"""Stable-key Iceberg MERGE statements."""


def statement(
    table: str, view: str, key: str, update_version: bool = False
) -> str:
    """Build a stable-key MERGE with optional cleanup-version upgrades."""
    prefix = (
        f'MERGE INTO {table} target USING {view} source '
        f'ON target.{key} = source.{key} '
    )
    update = (
        'WHEN MATCHED AND source.cleanup_version > target.cleanup_version '
        'THEN UPDATE SET * '
        if update_version
        else ''
    )
    return prefix + update + 'WHEN NOT MATCHED THEN INSERT *'
