"""Decide promotion by comparing a candidate against the untuned base.

The v2 model improved validation loss from 3.4744 to 1.1585 while its
code-fix pass rate fell from 0.75 to 0.00 and half of all short prompts
returned nothing. Loss is therefore not a promotion signal; behaviour is.
"""

import argparse
import json

from pathlib import Path

from .gate_rules import decide


def main() -> None:
    """Emit a pass or fail verdict with the reasons that produced it."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--base-bench', type=Path, required=True)
    parser.add_argument('--candidate-bench', type=Path, required=True)
    parser.add_argument('--base-empty', type=Path, required=True)
    parser.add_argument('--candidate-empty', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    verdict = decide(
        _load(values.base_bench),
        _load(values.candidate_bench),
        _load(values.base_empty),
        _load(values.candidate_empty),
    )
    values.output.write_text(
        json.dumps(verdict, indent=2, sort_keys=True) + '\n'
    )
    print(json.dumps(verdict, sort_keys=True))
    if not verdict['promote']:
        raise SystemExit('promotion gate failed')


def _load(path: Path) -> dict[str, object]:
    """Return one benchmark report."""
    value = json.loads(path.read_text())
    if not isinstance(value, dict):
        raise TypeError(f'expected an object in {path}')
    return value


if __name__ == '__main__':
    main()
