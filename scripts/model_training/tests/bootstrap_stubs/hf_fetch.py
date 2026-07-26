"""Stub dataset fetch used to verify bootstrap path wiring offline."""

import argparse
import json

from pathlib import Path


FILES = ('train-pairs.jsonl', 'validation-pairs.jsonl')


def main() -> None:
    """Write tiny valid splits into the state directory."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo', required=True)
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    values.output.mkdir(parents=True, exist_ok=True)
    for name in FILES:
        record = {'prompt': 'p', 'completion': 'c'}
        (values.output / name).write_text(json.dumps(record) + '\n')
    print(json.dumps({'stub': True, 'output': str(values.output)}))


if __name__ == '__main__':
    main()
