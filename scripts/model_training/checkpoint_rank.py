"""Rank training checkpoints by observed behaviour."""

MAX_EMPTY_RATE = 0.02


def rank(records: list[dict[str, object]]) -> dict[str, object]:
    """Return the best checkpoint, preferring tool use over silence.

    A checkpoint that answers nothing is useless regardless of its loss, so
    candidates above the empty-response budget are excluded before ranking
    by tool-call rate.
    """
    usable = [
        record
        for record in records
        if float(record.get('empty_rate', 1.0)) <= MAX_EMPTY_RATE
    ]
    pool = usable or records
    best = max(pool, key=_key)
    return {
        'best_step': int(best.get('step', 0)),
        'tool_call_rate': best.get('tool_call_rate'),
        'empty_rate': best.get('empty_rate'),
        'candidates': len(records),
        'within_empty_budget': len(usable),
        'rule': 'exclude silent checkpoints, then maximise tool-call rate',
        'degraded': not usable,
    }


def _key(record: dict[str, object]) -> tuple[float, float]:
    """Return a sort key favouring tool use and penalising silence."""
    return (
        float(record.get('tool_call_rate', 0.0)),
        -float(record.get('empty_rate', 1.0)),
    )
