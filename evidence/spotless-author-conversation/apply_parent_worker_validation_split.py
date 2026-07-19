"""Extract worker-target validation from the oversized external controller."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/a2a_server")
MONITOR = ROOT / "monitor_api.py"
HEARTBEAT = ROOT / "worker_heartbeat.py"
FAILURE = ROOT / "worker_target_failure.py"
VALIDATION = ROOT / "worker_target_validation.py"
HEARTBEAT_CONTENT = '''"""Worker heartbeat freshness policy."""

from datetime import datetime, timezone
from typing import Optional


def is_recent(value: Optional[str], max_age_seconds: int = 120) -> bool:
    """Return whether a serialized heartbeat is within the active window."""
    if not value:
        return False
    try:
        normalized = value.replace('Z', '+00:00')
        last = datetime.fromisoformat(normalized)
        if last.tzinfo is None:
            last = last.replace(tzinfo=timezone.utc)
        return (datetime.now(timezone.utc) - last).total_seconds() <= max_age_seconds
    except ValueError:
        return False
'''
FAILURE_CONTENT = '''"""Strict and fallback outcomes for unavailable target workers."""

import logging
from typing import MutableMapping, Optional

from fastapi import HTTPException

logger = logging.getLogger(__name__)


def reject(
    metadata: MutableMapping[str, object],
    target: str,
    strict: bool,
    heartbeat: Optional[str] = None,
) -> None:
    """Raise in strict mode or remove an unavailable advisory target."""
    if heartbeat is None:
        detail = f'Target worker "{target}" is not connected.'
        log_message = f'Target worker "{target}" is not connected; falling back to auto-select'
        warning = f'Target worker "{target}" is not connected; task auto-routed to available worker.'
    else:
        detail = f'Target worker "{target}" has a stale heartbeat.'
        log_message = f'Target worker "{target}" has stale heartbeat ({heartbeat}); falling back to auto-select'
        warning = f'Target worker "{target}" is stale (last heartbeat: {heartbeat}); task auto-routed.'
    if strict:
        raise HTTPException(status_code=409, detail=detail)
    logger.info(log_message)
    metadata.pop('target_worker_id', None)
    metadata['_routing_warning'] = warning
'''
VALIDATION_CONTENT = '''"""Availability validation for explicitly routed workers."""

import logging
from typing import MutableMapping, Optional

from . import database as db
from .worker_heartbeat import is_recent
from .worker_target_failure import reject

logger = logging.getLogger(__name__)


async def validate_target_worker(
    metadata: MutableMapping[str, object], *, strict: bool = False
) -> None:
    """Require a live worker in strict mode or remove an advisory route."""
    target = str(metadata.get('target_worker_id') or '').strip()
    if not target:
        return
    worker = await _load(target)
    if not worker:
        reject(metadata, target, strict)
        return
    heartbeat = worker.get('last_seen') or worker.get('last_heartbeat')
    value = str(heartbeat) if heartbeat else None
    if not is_recent(value):
        reject(metadata, target, strict, value or 'missing')


async def _load(target: str) -> Optional[dict[str, object]]:
    try:
        return await db.db_get_worker(target)
    except Exception as error:
        logger.debug('Failed to load target worker %s from DB: %s', target, error)
        return None
'''


def main() -> None:
    """Write focused modules and shrink the legacy controller."""
    HEARTBEAT.write_text(HEARTBEAT_CONTENT)
    FAILURE.write_text(FAILURE_CONTENT)
    VALIDATION.write_text(VALIDATION_CONTENT)
    text = MONITOR.read_text()
    start_marker = 'def _is_recent_heartbeat(\n'
    end_marker = 'async def get_current_policy_user(request: Request) -> dict:\n'
    if start_marker in text:
        start = text.index(start_marker)
        end = text.index(end_marker, start)
        text = text[:start] + end_marker + text[end + len(end_marker):]
    import_line = (
        'from .worker_target_validation import '
        'validate_target_worker as _validate_target_worker_is_available\n'
    )
    if import_line not in text:
        anchor = 'from .task_orchestration import orchestrate_task_route\n'
        if text.count(anchor) != 1:
            raise RuntimeError('worker validation import anchor is missing')
        text = text.replace(anchor, anchor + import_line, 1)
    MONITOR.write_text(text)


if __name__ == "__main__":
    main()