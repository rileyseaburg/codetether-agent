"""Sharded source-object loading for distributed Spark transforms."""

from functools import partial

from .s3_listing import keys
from .s3_reader import partition
from .settings import Settings


def objects(spark: object, settings: Settings) -> object:
    """Return `(URI, text)` pairs using sharded S3 discovery and reads."""
    discovered = keys(settings)
    partitions = max(1, min(len(discovered), settings.min_partitions))
    paths = spark.sparkContext.parallelize(discovered, partitions)
    if (
        settings.apply
        and not settings.reprocess
        and spark.catalog.tableExists(settings.tables.manifests)
    ):
        paths = _unseen(spark, paths, settings)
    reader = partial(
        partition, endpoint=settings.endpoint, bucket=settings.bucket
    )
    return paths.mapPartitions(reader)


def _unseen(spark: object, paths: object, settings: Settings) -> object:
    from pyspark.sql.types import StringType, StructField, StructType

    schema = StructType([StructField('key', StringType(), False)])
    incoming = spark.createDataFrame(paths.map(lambda key: (key,)), schema)
    known = spark.table(settings.tables.manifests).selectExpr(
        f"replace(source_uri, 's3a://{settings.bucket}/', '') AS key"
    )
    return incoming.join(known, 'key', 'left_anti').rdd.map(lambda row: row.key)
