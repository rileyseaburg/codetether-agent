"""Verify the target architectures are importable.

Run at image build time so a missing architecture fails the build in
seconds rather than after a GPU has been provisioned. `qwen3_5` is absent
from transformers 4.57.1, 4.58.0, 4.60.0, and 5.0.0.
"""

import sys

from transformers.models.auto.configuration_auto import CONFIG_MAPPING_NAMES


REQUIRED = ('qwen3_5', 'qwen3_5_moe')


def main() -> None:
    """Exit non-zero when a required architecture is unavailable."""
    missing = [name for name in REQUIRED if name not in CONFIG_MAPPING_NAMES]
    if missing:
        sys.stderr.write(f'missing architectures: {missing}\n')
        raise SystemExit(1)
    sys.stdout.write(f'architectures available: {list(REQUIRED)}\n')


if __name__ == '__main__':
    main()
