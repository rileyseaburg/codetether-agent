"""Build the manifest summary for one rendered dataset split."""

from dataclasses import dataclass
from pathlib import Path


@dataclass
class Totals:
    """Counters accumulated while rendering a split."""

    included: int
    excluded: int
    tokens: int
    maximum: int
    with_tools: int


def summarize(
    source: Path, target: Path, totals: Totals, digest: str
) -> dict[str, object]:
    """Return the manifest entry describing a rendered split."""
    mean = round(totals.tokens / totals.included, 2) if totals.included else 0.0
    return {
        'source': str(source),
        'path': str(target),
        'included': totals.included,
        'excluded_over_limit': totals.excluded,
        'rendered_with_tools': totals.with_tools,
        'tokens': totals.tokens,
        'mean_tokens': mean,
        'max_tokens': totals.maximum,
        'bytes': target.stat().st_size,
        'sha256': digest,
    }
