"""Install the CodeTether CLI inside an ephemeral training runtime."""

import shutil
import subprocess


CRATE = 'codetether-agent'


def install() -> dict[str, object]:
    """Install the CLI with cargo when it is not already present."""
    if shutil.which('codetether'):
        return {'action': 'already-present'}
    if not shutil.which('cargo'):
        return {'action': 'skipped', 'reason': 'cargo unavailable'}
    result = subprocess.run(
        ['cargo', 'install', CRATE, '--locked'],
        capture_output=True,
        text=True,
        check=False,
    )
    return {
        'action': 'cargo-install',
        'returncode': result.returncode,
        'stderr_tail': result.stderr.strip().splitlines()[-3:],
    }


def version() -> str | None:
    """Return the installed CLI version, or None when absent."""
    if not shutil.which('codetether'):
        return None
    result = subprocess.run(
        ['codetether', '--version'],
        capture_output=True,
        text=True,
        check=False,
    )
    return result.stdout.strip() or None
