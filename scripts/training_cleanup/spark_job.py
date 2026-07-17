"""Distributed Spark driver for historical training-data cleanup."""

from .run_summary import build as summary
from .settings_loader import load
from .spark_output import write
from .spark_session import create
from .spark_source import objects
from .spark_transform import object_lines


def main() -> None:
    """Run a dry-run audit or write immutable cleaned datasets."""
    settings = load()
    spark = create(settings)
    try:
        source = objects(spark, settings)
        tagged = source.flatMap(
            lambda item: object_lines(
                item, settings.max_content_chars, settings.run_id
            )
        ).persist()
        counts = tagged.countByKey()
        if settings.apply:
            write(spark, tagged, settings)
        print(summary(settings, counts))
        tagged.unpersist()
    finally:
        spark.stop()
