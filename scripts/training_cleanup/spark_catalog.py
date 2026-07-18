"""Apache Polaris-backed Iceberg Spark catalog configuration."""

import os

from .settings import Settings


def configure(builder: object, settings: Settings) -> object:
    """Attach the authenticated Iceberg REST catalog to a Spark builder."""
    prefix = f'spark.sql.catalog.{settings.catalog}'
    values = {
        '': 'org.apache.iceberg.spark.SparkCatalog',
        '.type': 'rest',
        '.uri': _required('POLARIS_URI'),
        '.warehouse': os.getenv('POLARIS_WAREHOUSE', 'codetether'),
        '.credential': f'{_required("POLARIS_PIPELINE_ID")}:'
        f'{_required("POLARIS_PIPELINE_SECRET")}',
        '.oauth2-server-uri': os.getenv(
            'POLARIS_OAUTH_URI',
            'http://polaris:8181/api/catalog/v1/oauth/tokens',
        ),
        '.scope': 'PRINCIPAL_ROLE:ALL',
        '.header.Polaris-Realm': os.getenv('POLARIS_REALM', 'POLARIS'),
        '.header.X-Iceberg-Access-Delegation': 'vended-credentials',
        '.client.region': os.getenv('AWS_REGION', 'us-east-1'),
    }
    builder = builder.config(
        'spark.sql.extensions',
        'org.apache.iceberg.spark.extensions.IcebergSparkSessionExtensions',
    )
    for suffix, value in values.items():
        builder = builder.config(f'{prefix}{suffix}', value)
    return builder


def _required(name: str) -> str:
    value = os.getenv(name)
    if not value:
        raise RuntimeError(f'{name} is required')
    return value
