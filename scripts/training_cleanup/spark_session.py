"""Spark session and MinIO S3A connector configuration."""

import os

from .settings import Settings
from .spark_catalog import configure


def create(settings: Settings) -> object:
    """Create a Spark session configured for path-style MinIO access."""
    from pyspark.sql import SparkSession

    builder = SparkSession.builder.appName('codetether-training-cleanup')
    spark = configure(builder, settings).getOrCreate()
    hadoop = spark.sparkContext._jsc.hadoopConfiguration()
    hadoop.set('fs.s3a.endpoint', settings.endpoint)
    hadoop.set('fs.s3a.access.key', _required('MINIO_ACCESS_KEY'))
    hadoop.set('fs.s3a.secret.key', _required('MINIO_SECRET_KEY'))
    hadoop.set('fs.s3a.path.style.access', 'true')
    hadoop.set('fs.s3a.connection.ssl.enabled', _ssl(settings.endpoint))
    hadoop.set(
        'fs.s3a.aws.credentials.provider',
        'org.apache.hadoop.fs.s3a.SimpleAWSCredentialsProvider',
    )
    hadoop.set('mapreduce.input.fileinputformat.input.dir.recursive', 'true')
    return spark


def _ssl(endpoint: str) -> str:
    return str(endpoint.startswith('https')).lower()


def _required(name: str) -> str:
    value = os.getenv(name)
    if not value:
        raise RuntimeError(f'{name} is required')
    return value
