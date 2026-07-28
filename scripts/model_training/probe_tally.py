"""Accumulate probe outcomes into reportable rates."""

from dataclasses import dataclass, field

from .probe_tools import TOOL_SCHEMA
from .schema_grade import grade


CALL_MARKERS = ('<tool_call>', '<function=')


@dataclass
class Tally:
    """Running counts across generated probe completions."""

    empty: int = 0
    syntactic: int = 0
    known: int = 0
    invented: int = 0
    params: int = 0
    schema: list[dict[str, object]] = field(default_factory=lambda: TOOL_SCHEMA)

    def record(self, text: str) -> None:
        """Fold one completion into the running counts."""
        if not text.strip():
            self.empty += 1
        if any(marker in text for marker in CALL_MARKERS):
            self.syntactic += 1
        verdict = grade(text, self.schema)
        self.known += int(verdict['known_tool'])
        self.invented += int(verdict['invented_tool'])
        self.params += int(verdict['params_valid'])

    def report(self, total: int) -> dict[str, object]:
        """Return rates over the probe set."""
        divisor = max(total, 1)
        return {
            'probes': total,
            'empty_rate': round(self.empty / divisor, 4),
            'tool_call_rate': round(self.syntactic / divisor, 4),
            'known_tool_rate': round(self.known / divisor, 4),
            'invented_tool_rate': round(self.invented / divisor, 4),
            'valid_params_rate': round(self.params / divisor, 4),
        }
