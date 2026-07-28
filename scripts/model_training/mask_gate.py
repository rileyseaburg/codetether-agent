"""Stop a run only when label masking is systemically broken.

Blocking on any fully masked example failed a healthy run: 5 of 200 sampled
pairs, 2.5 percent, had prompts near the sequence limit and lost their
completion to truncation. TRL drops those rows itself, so a small rate is
expected rather than a defect.
"""

MAX_FULLY_MASKED_RATE = 0.10
MIN_SUPERVISED_FRACTION = 0.005


def gate(report: dict[str, object]) -> None:
    """Raise when masking indicates a systemic failure."""
    sampled = max(int(report['pairs_sampled']), 1)
    masked_rate = int(report['fully_masked']) / sampled
    supervised = float(report['supervised_fraction'])
    if masked_rate > MAX_FULLY_MASKED_RATE:
        raise SystemExit(
            f'fully masked rate {masked_rate:.1%} exceeds '
            f'{MAX_FULLY_MASKED_RATE:.0%}'
        )
    if supervised < MIN_SUPERVISED_FRACTION:
        raise SystemExit(
            f'supervised fraction {supervised:.4f} below '
            f'{MIN_SUPERVISED_FRACTION}'
        )
