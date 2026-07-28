"""Choose a checkpoint by behaviour rather than validation loss.

The v2 model reached its lowest loss while producing no tool calls and empty
replies to half of all short prompts. Loss alone therefore cannot select a
checkpoint; the behaviour trace recorded during training decides.
"""

import argparse
import json

from pathlib import Path

from .checkpoint_rank import rank


def main() -> None:
    """Print the best checkpoint step and the reason it was chosen."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--behaviour', type=Path, required=True)
    parser.add_argument('--output', type=Path)
    values = parser.parse_args()
    records = _load(values.behaviour)
    if not records:
        raise SystemExit(f'no behaviour records in {values.behaviour}')
    verdict = rank(records)
    text = json.dumps(verdict, indent=2, sort_keys=True) + '\n'
    if values.output:
        values.output.write_text(text)
    print(text)


def _load(path: Path) -> list[dict[str, object]]:
    """Return every behaviour record written during training."""
    records: list[dict[str, object]] = []
    for line in path.read_text().splitlines():
        if not line.strip():
            continue
        value = json.loads(line)
        if isinstance(value, dict):
            records.append(value)
    return records


if __name__ == '__main__':
    main()
