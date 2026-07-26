"""Inspect a running process through /proc without shell parsing."""

import os

from pathlib import Path


def describe(pid: int) -> dict[str, object]:
    """Return liveness, working directory, and deleted-file evidence."""
    root = Path('/proc') / str(pid)
    if not root.exists():
        return {'pid': pid, 'running': False}
    cwd = _link(root / 'cwd')
    return {
        'pid': pid,
        'running': _alive(pid),
        'cwd': cwd,
        'deleted_cwd': bool(cwd) and cwd.endswith('(deleted)'),
        'deleted_state': _has_deleted_files(root)
        or (bool(cwd) and cwd.endswith('(deleted)')),
        'rss_bytes': _rss(root),
    }


def _alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return True


def _link(path: Path) -> str | None:
    try:
        return str(path.readlink())
    except OSError:
        return None


def _has_deleted_files(root: Path) -> bool:
    """Return whether the process holds handles on deleted paths."""
    try:
        entries = list((root / 'fd').iterdir())
    except OSError:
        return False
    return any('(deleted)' in (_link(e) or '') for e in entries)


def _rss(root: Path) -> int | None:
    try:
        fields = (root / 'statm').read_text().split()
    except OSError:
        return None
    return int(fields[1]) * os.sysconf('SC_PAGE_SIZE')
