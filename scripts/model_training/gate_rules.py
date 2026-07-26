"""Promotion criteria for a fine-tuned candidate model."""

MAX_EMPTY_RATE = 0.02
MIN_TOOL_CALL_RATE = 0.10


def decide(
    base: dict[str, object],
    candidate: dict[str, object],
    base_empty: dict[str, object],
    candidate_empty: dict[str, object],
) -> dict[str, object]:
    """Return a promotion verdict with per-criterion results."""
    checks = [
        _check(
            'code_pass_rate_not_worse',
            float(candidate['code_pass_rate']) >= float(base['code_pass_rate']),
            base['code_pass_rate'],
            candidate['code_pass_rate'],
        ),
        _check(
            'empty_rate_within_budget',
            float(candidate_empty['empty_rate']) <= MAX_EMPTY_RATE,
            base_empty['empty_rate'],
            candidate_empty['empty_rate'],
        ),
        _check(
            'tool_calls_emitted',
            float(candidate['tool_call_rate']) >= MIN_TOOL_CALL_RATE,
            base['tool_call_rate'],
            candidate['tool_call_rate'],
        ),
    ]
    return {
        'promote': all(c['passed'] for c in checks),
        'checks': checks,
        'rule': 'behavioural comparison against the untuned base model',
    }


def _check(
    name: str, passed: bool, base: object, candidate: object
) -> dict[str, object]:
    return {
        'name': name,
        'passed': passed,
        'base': base,
        'candidate': candidate,
    }
