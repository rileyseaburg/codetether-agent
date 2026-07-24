"""Cross-object conversation-cleaning results."""

from dataclasses import dataclass, field

from .model import JsonObject, RejectedRecord


@dataclass
class GroupResult:
    """Trainer samples and rejected records for one correlation group."""

    samples: list[JsonObject] = field(default_factory=list)
    rejected: list[RejectedRecord] = field(default_factory=list)
