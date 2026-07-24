"""Discover the newest snapshot for every local session identifier."""

import json
import os

from pathlib import Path


_SKIP = {'.git', '.venv', 'node_modules', 'target', 'venv'}


def canonical(home: Path) -> dict[str, Path]:
    """Return one newest, fullest snapshot path per session ID."""
    selected: dict[str, tuple[tuple[str, int, int], Path]] = {}
    for root, directories, files in os.walk(home):
        directories[:] = [name for name in directories if name not in _SKIP]
        if not root.endswith('/.codetether-agent/sessions'):
            continue
        for name in files:
            if not name.endswith('.json') or name.endswith('.journal.jsonl'):
                continue
            path = Path(root, name)
            value = json.loads(path.read_text())
            session_id = str(value.get('id') or path.stem)
            score = (
                str(value.get('updated_at') or ''),
                len(value.get('messages') or []),
                path.stat().st_size,
            )
            if session_id not in selected or score > selected[session_id][0]:
                selected[session_id] = (score, path)
    return {key: value[1] for key, value in selected.items()}
