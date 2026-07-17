"""Retry-safe Iceberg writes."""

from .iceberg_schema import MANIFESTS, QUARANTINE, SAMPLES
from .iceberg_tables import ensure, reject_completed_run
from .merge_sql import statement
from .settings import Settings


_OUTPUTS = (
    ('samples', SAMPLES, 'sample_id'),
    ('quarantine', QUARANTINE, 'quarantine_id'),
    ('manifests', MANIFESTS, 'source_uri'),
)


def write(spark: object, tagged: object, settings: Settings) -> None:
    """Merge one run into format-version 2 Iceberg tables."""
    ensure(spark, settings)
    reject_completed_run(spark, settings)
    for kind, schema, key in _OUTPUTS:
        lines = tagged.filter(lambda item, name=kind: item[0] == name).values()
        if lines.isEmpty():
            continue
        frame = spark.read.schema(schema).json(lines)
        view = f'training_cleanup_{kind}'
        frame.createOrReplaceTempView(view)
        table = getattr(settings.tables, kind)
        spark.sql(statement(table, view, key))
