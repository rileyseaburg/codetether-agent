"""Creation and completed-run checks for Iceberg tables."""

from .iceberg_schema import TABLES
from .settings import Settings


def ensure(spark: object, settings: Settings) -> None:
    """Create the namespace and format-version 2 tables when absent."""
    namespace = f'{settings.catalog}.{settings.namespace}'
    spark.sql(f'CREATE NAMESPACE IF NOT EXISTS {namespace}')
    names = settings.tables
    for attribute, columns, partitions in TABLES:
        table = getattr(names, attribute)
        spark.sql(
            f'CREATE TABLE IF NOT EXISTS {table} ({columns}) '
            f'USING iceberg PARTITIONED BY ({partitions}) '
            "TBLPROPERTIES ('format-version'='2')"
        )


def reject_completed_run(spark: object, settings: Settings) -> None:
    """Stop before writes when the manifest already contains this run ID."""
    escaped = settings.run_id.replace("'", "''")
    table = settings.tables.manifests
    if spark.sql(
        f"SELECT 1 FROM {table} WHERE run_id = '{escaped}' LIMIT 1"
    ).take(1):
        raise RuntimeError(f'cleanup run already completed: {settings.run_id}')
