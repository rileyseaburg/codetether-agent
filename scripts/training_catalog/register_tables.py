"""Recover Polaris table registrations from immutable Iceberg metadata."""

import os

from training_cleanup.settings import Settings
from training_cleanup.spark_session import create

from training_catalog.latest_metadata import find


_TABLES = (
    'training_samples',
    'training_quarantine',
    'training_cleanup_manifests',
)


def main() -> None:
    """Register existing table metadata without rewriting data files."""
    settings = _settings()
    spark = create(settings)
    try:
        spark.sql('CREATE NAMESPACE IF NOT EXISTS iceberg.codetether')
        for table in _TABLES:
            qualified = f'iceberg.codetether.{table}'
            if spark.catalog.tableExists(qualified):
                continue
            metadata = find(settings.endpoint, settings.bucket, table)
            spark.sql(
                'CALL iceberg.system.register_table('
                f"table => 'codetether.{table}', "
                f"metadata_file => '{metadata}')"
            )
            print(f'Registered {qualified} from {metadata}')
    finally:
        spark.stop()


def _settings() -> Settings:
    endpoint = os.environ['MINIO_ENDPOINT']
    return Settings(
        endpoint,
        'codetether-training',
        'training/v2',
        'training/v2/',
        'iceberg',
        'codetether',
        'training',
        'catalog-recovery',
        32_768,
        65_536,
        96,
        1,
        False,
        False,
    )


if __name__ == '__main__':
    main()
