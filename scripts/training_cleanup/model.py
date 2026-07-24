"""Typed values shared by the training cleanup pipeline."""

from dataclasses import dataclass, field
from typing import TypeAlias


JsonObject: TypeAlias = dict[str, object]


@dataclass(frozen=True)
class SourceRecord:
    """One parsed source line with its original position."""

    line: int
    value: JsonObject
    source_uri: str = ''
    source_sha256: str = ''


@dataclass(frozen=True)
class RejectedRecord:
    """A source line excluded from model-training output."""

    line: int
    reason: str
    value: object
    source_uri: str = ''
    source_sha256: str = ''


@dataclass
class CleanResult:
    """Accepted samples and audit-preserving quarantine entries."""

    samples: list[JsonObject] = field(default_factory=list)
    rejected: list[RejectedRecord] = field(default_factory=list)
    input_lines: int = 0

    @property
    def accepted_records(self) -> int:
        """Count source records represented in accepted samples."""
        return sum(_source_line_count(sample) for sample in self.samples)


def _source_line_count(sample: JsonObject) -> int:
    metadata = sample.get('metadata')
    if not isinstance(metadata, dict):
        return 0
    lines = metadata.get('source_lines')
    return len(lines) if isinstance(lines, list) else 0
