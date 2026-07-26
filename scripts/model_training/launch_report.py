"""Report a verified training launch.

A printed PID alone is weak evidence: a stale identifier with no log file
looks identical to a healthy start. This confirms the process is alive and
the log exists before declaring the run launched.
"""

import argparse
import json
import os

from pathlib import Path


def alive(pid: int) -> bool:
    """Return whether the process is currently running."""
    try:
        os.kill(pid, 0)
    except (OSError, ProcessLookupError):
        return False
    return True


def main() -> None:
    """Print launch evidence, exiting non-zero when the run is not live."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--pid', type=int, required=True)
    parser.add_argument('--log', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    values = parser.parse_args()
    running = alive(values.pid)
    report = {
        'pid': values.pid,
        'running': running,
        'log': str(values.log),
        'log_exists': values.log.exists(),
        'output': str(values.output),
    }
    print(json.dumps(report, sort_keys=True))
    if not running or not report['log_exists']:
        raise SystemExit('training process did not start')


if __name__ == '__main__':
    main()
