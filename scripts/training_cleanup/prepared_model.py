"""Prepared immutable source-object values."""

from dataclasses import dataclass

from .model import RejectedRecord, SourceRecord


@dataclass(frozen=True)
class PreparedObject:
    """Record candidates and early rejections from one raw object."""

    source_uri: str
    source_sha256: str
    input_lines: int
    records: list[SourceRecord]
    rejected: list[RejectedRecord]
