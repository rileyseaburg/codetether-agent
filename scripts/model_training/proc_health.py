"""Exit zero when a training process is healthy enough to protect.

Deleting the code directory leaves the trainer running against deleted
inodes: it produces no readable log and makes no progress, yet a naive
liveness check still reports it as active and blocks relaunching.
"""

import argparse

from .proc_probe import describe


def main() -> None:
    """Exit zero when the process is running and not orphaned."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--pid', type=int, required=True)
    values = parser.parse_args()
    state = describe(values.pid)
    healthy = bool(state.get('running')) and not state.get('deleted_state')
    raise SystemExit(0 if healthy else 1)


if __name__ == '__main__':
    main()
