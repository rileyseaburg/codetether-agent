"""Redact reusable author-session capabilities from public task polling."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
MODULE = ROOT / 'a2a_server/forgejo_task_response.py'
MONITOR = ROOT / 'a2a_server/monitor_api.py'
TEST = ROOT / 'tests/test_forgejo_task_response.py'
MODULE_CONTENT = '''"""Public response projection for Forgejo author tasks."""

from collections.abc import Mapping

PROTOCOL = 'codetether.forgejo-author.v1'
SENSITIVE = {
    'resume_session_id',
    'author_provenance_id',
    'author_agent_identity',
    'target_worker_id',
    'author_identity_key_id',
    'server_author_binding_verified',
    'tenant_id',
    'idempotency_key',
    'github_work_key',
    'context_id',
    'conversation_id',
}


def public(task: Mapping[str, object]) -> dict[str, object]:
    """Remove private continuation capabilities from a public task response."""
    result = dict(task)
    metadata = result.get('metadata')
    if not isinstance(metadata, dict) or metadata.get('protocol') != PROTOCOL:
        return result
    result['metadata'] = {
        key: value for key, value in metadata.items() if key not in SENSITIVE
    }
    result['session_id'] = None
    return result
'''
TEST_CONTENT = '''from a2a_server.forgejo_task_response import public


def test_public_author_task_hides_reusable_session_capabilities():
    task = {
        'id': 'cttask_public',
        'session_id': 'runtime-session',
        'metadata': {
            'protocol': 'codetether.forgejo-author.v1',
            'resume_session_id': 'author-session',
            'author_provenance_id': 'ctprov_secret',
            'tenant_id': 'tenant-secret',
            'context_id': 'conversation-secret',
            'blocking_findings': 1,
        },
    }
    response = public(task)
    assert response['session_id'] is None
    assert response['metadata'] == {
        'protocol': 'codetether.forgejo-author.v1',
        'blocking_findings': 1,
    }


def test_non_protocol_task_response_is_unchanged():
    task = {'id': 'legacy', 'metadata': {'resume_session_id': 'internal'}}
    assert public(task) == task
'''


def main() -> None:
    """Install the response projection at the public single-task endpoint."""
    MODULE.write_text(MODULE_CONTENT)
    text = MONITOR.read_text()
    import_line = 'from .forgejo_task_response import public as public_forgejo_task\n'
    if import_line not in text:
        anchor = 'from .forgejo_request_scope import resolve as forgejo_request_scope\n'
        text = text.replace(anchor, anchor + import_line, 1)
    old = '    return task.to_dict()\n\n\n@agent_router_alias.post(\'/tasks/{task_id}/cancel\')\n'
    new = '    return public_forgejo_task(task.to_dict())\n\n\n@agent_router_alias.post(\'/tasks/{task_id}/cancel\')\n'
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('public task response anchor is missing')
        text = text.replace(old, new, 1)
    MONITOR.write_text(text)
    TEST.write_text(TEST_CONTENT)


if __name__ == '__main__':
    main()