"""Typed Spark cleanup command settings and argument parsing."""

from dataclasses import dataclass

from .table_names import TableNames, names


@dataclass(frozen=True)
class Settings:
    """Validated source, destination, and execution policy."""

    endpoint: str
    bucket: str
    source_prefix: str
    source_before: str
    catalog: str
    namespace: str
    table_prefix: str
    run_id: str
    max_content_chars: int
    max_sample_chars: int
    max_sample_messages: int
    min_partitions: int
    apply: bool
    reprocess: bool

    @property
    def source_uri(self) -> str:
        """Return the immutable S3A source directory."""
        return f's3a://{self.bucket}/{self.source_prefix.strip("/")}/'

    @property
    def cutoff_uri(self) -> str:
        """Return the exclusive immutable source-key cutoff."""
        return f's3a://{self.bucket}/{self.source_before.strip("/")}/'

    @property
    def tables(self) -> TableNames:
        """Return the qualified Iceberg table names."""
        return names(self.catalog, self.namespace, self.table_prefix)
