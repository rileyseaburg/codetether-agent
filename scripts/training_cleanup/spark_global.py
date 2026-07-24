"""Distributed cross-object reconstruction and output assembly."""

from functools import partial

from .correlation import pairs, uncorrelated
from .global_rows import quarantine, sample
from .group_clean import clean
from .prepare_object import build as prepare
from .row_common import cleaned_at
from .settings import Settings
from .spark_manifests import rows as manifest_rows
from .spark_storage import disk_only


def lines(source: object, settings: Settings) -> object:
    """Return tagged sample, quarantine, and manifest rows."""
    timestamp = cleaned_at()
    prepared = source.map(
        partial(prepare, max_content_chars=settings.max_content_chars)
    ).persist(disk_only())
    cleaner = partial(
        clean,
        max_messages=settings.max_sample_messages,
        max_characters=settings.max_sample_chars,
    )
    groups = (
        prepared.flatMap(pairs).groupByKey().map(cleaner).persist(disk_only())
    )
    samples = groups.flatMap(lambda result: result.samples)
    rejected = _rejections(prepared, groups)
    manifests = manifest_rows(prepared, samples, rejected, settings, timestamp)
    tagged = samples.map(
        lambda value: sample(value, settings.run_id, timestamp)
    ).union(
        rejected.map(
            lambda value: quarantine(value, settings.run_id, timestamp)
        )
    )
    return tagged.union(manifests)


def _rejections(prepared: object, groups: object) -> object:
    early = prepared.flatMap(lambda value: value.rejected)
    missing = prepared.flatMap(uncorrelated)
    grouped = groups.flatMap(lambda result: result.rejected)
    return early.union(missing).union(grouped)
