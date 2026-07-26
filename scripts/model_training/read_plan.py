"""Read the recommended model from a device plan file."""

import argparse
import json

from pathlib import Path


def main() -> None:
    """Print the recommended model identifier."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--path', type=Path, required=True)
    parser.add_argument('--field', default='recommended_model')
    values = parser.parse_args()
    plan = json.loads(values.path.read_text())
    print(plan[values.field], end='')


if __name__ == '__main__':
    main()
