"""Print the Vertex machine and accelerator for a base model."""

import argparse

from .vertex_machine import select


def main() -> None:
    """Emit `machine accelerator` for shell consumption."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--model', required=True)
    values = parser.parse_args()
    machine, accelerator = select(values.model)
    print(f'{machine} {accelerator}')


if __name__ == '__main__':
    main()
