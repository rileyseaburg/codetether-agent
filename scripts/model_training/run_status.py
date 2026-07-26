"""Report whether a training run is alive, wedged, or finished.

A registered PID with no log file is ambiguous: the process may be writing
to deleted inodes after its state directory was replaced. Reading
`/proc/<pid>` distinguishes these cases from observation rather than guess.
"""

import argparse
import json

from pathlib import Path

from .proc_probe import describe
from .run_artifacts import survey


def main() -> None:
    """Print a single verdict covering process and artifact state."""
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--state', type=Path, default=Path('/content/codetether-state')
    )
    parser.add_argument('--pid', type=int)
    values = parser.parse_args()
    artifacts = survey(values.state)
    process = describe(values.pid) if values.pid else {'pid': None}
    verdict = _verdict(process, artifacts)
    print(
        json.dumps(
            {'verdict': verdict, 'process': process, 'artifacts': artifacts},
            indent=2,
            sort_keys=True,
        )
    )


def _verdict(process: dict[str, object], artifacts: dict[str, object]) -> str:
    """Return a plain-language conclusion."""
    if artifacts['final_adapter']:
        return 'finished: final adapter present'
    if not process.get('running'):
        checkpoints = artifacts['checkpoints']
        if checkpoints:
            return f'stopped with {len(checkpoints)} checkpoint(s); resumable'
        return 'stopped with no checkpoint; restart required'
    if process.get('deleted_state'):
        return 'running but its state directory was deleted; restart required'
    if not artifacts['log_exists']:
        return 'running; log not yet created'
    return 'running normally'


if __name__ == '__main__':
    main()
