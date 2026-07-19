"""Resolve focused lint findings that require semantic edits."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
FAILURE = ROOT / "a2a_server/worker_target_failure.py"
TRANSPORT = ROOT / "tests/forgejo_verification_transport.py"
PRIVATE_TESTS = (
    ROOT / "tests/agent_bridge_fixtures.py",
    ROOT / "tests/test_agent_bridge_required_persistence.py",
    ROOT / "tests/test_agent_bridge_save_result.py",
    ROOT / "tests/test_forgejo_work_dedupe.py",
)


def main() -> None:
    """Wrap diagnostics, type the callback, and mark private-API tests."""
    text = FAILURE.read_text()
    replacements = {
        '''        log_message = f'Target worker "{target}" is not connected; falling back to auto-select'
''': '''        log_message = (
            f'Target worker "{target}" is not connected; '
            'falling back to auto-select'
        )
''',
        '''        warning = f'Target worker "{target}" is not connected; task auto-routed to available worker.'
''': '''        warning = (
            f'Target worker "{target}" is not connected; '
            'task auto-routed to available worker.'
        )
''',
        '''        log_message = f'Target worker "{target}" has stale heartbeat ({heartbeat}); falling back to auto-select'
''': '''        log_message = (
            f'Target worker "{target}" has stale heartbeat ({heartbeat}); '
            'falling back to auto-select'
        )
''',
        '''        warning = f'Target worker "{target}" is stale (last heartbeat: {heartbeat}); task auto-routed.'
''': '''        warning = (
            f'Target worker "{target}" is stale (last heartbeat: {heartbeat}); '
            'task auto-routed.'
        )
''',
    }
    for old, new in replacements.items():
        text = text.replace(old, new)
    FAILURE.write_text(text)
    transport = TRANSPORT.read_text()
    transport = transport.replace('    def respond(request):\n',
        '    def respond(request: httpx.Request) -> httpx.Response:\n')
    TRANSPORT.write_text(transport)
    prepare = ROOT / 'tests/test_forgejo_author_prepare.py'
    prepare.write_text(
        prepare.read_text().replace(
            '    task_id, existing = await prepare(value)\n',
            '    _task_id, existing = await prepare(value)\n',
        )
    )
    scope = ROOT / 'tests/test_forgejo_request_scope.py'
    scope_text = scope.read_text()
    scope_import = 'from starlette.status import HTTP_503_SERVICE_UNAVAILABLE\n'
    if scope_import not in scope_text:
        scope_text = scope_text.replace('from fastapi import HTTPException\n',
            'from fastapi import HTTPException\n' + scope_import)
    scope_text = scope_text.replace('assert error.value.status_code == 503',
        'assert error.value.status_code == HTTP_503_SERVICE_UNAVAILABLE')
    scope.write_text(scope_text)
    worker = ROOT / 'tests/test_worker_target_validation.py'
    worker_text = worker.read_text()
    worker_import = 'from starlette.status import HTTP_409_CONFLICT\n'
    if worker_import not in worker_text:
        worker_text = worker_text.replace('from fastapi import HTTPException\n',
            'from fastapi import HTTPException\n' + worker_import)
    worker_text = worker_text.replace('assert error.value.status_code == 409',
        'assert error.value.status_code == HTTP_409_CONFLICT')
    worker.write_text(worker_text)
    directive = '# ruff: noqa: SLF001\n'
    for path in PRIVATE_TESTS:
        text = path.read_text()
        if not text.startswith(directive):
            path.write_text(directive + text)


if __name__ == '__main__':
    main()