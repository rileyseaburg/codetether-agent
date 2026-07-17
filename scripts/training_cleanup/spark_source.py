"""Bulk recursive source-object loading for Spark."""

from typing import Protocol, cast

from .settings import Settings


class BinaryRow(Protocol):
    """Structural type for the selected Spark binary-file columns."""

    def __getitem__(self, key: str) -> object:
        """Return a selected column value."""
        ...


def objects(spark: object, settings: Settings) -> object:
    """Return `(URI, text)` pairs without per-directory S3 listings."""
    from pyspark.sql import functions as sql

    frame = (
        spark.read.format('binaryFile')
        .option('recursiveFileLookup', 'true')
        .option('pathGlobFilter', '*.jsonl')
        .load(settings.source_uri)
        .filter(sql.col('path') < sql.lit(settings.cutoff_uri))
        .select('path', 'content')
    )
    if settings.apply and spark.catalog.tableExists(settings.tables.manifests):
        known = spark.table(settings.tables.manifests).selectExpr(
            'source_uri AS path'
        )
        frame = frame.join(known, 'path', 'left_anti')
    return frame.rdd.map(_text_pair).repartition(settings.min_partitions)


def _text_pair(row: BinaryRow) -> tuple[str, str]:
    binary = cast(bytes | bytearray, row['content'])
    content = bytes(binary).decode('utf-8', errors='replace')
    return cast(str, row['path']), content
