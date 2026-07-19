"""Keep Forgejo idempotency inside its tenant-scoped protocol boundary."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
POOL = ROOT / "a2a_server/persistent_worker_pool.py"
TASK = ROOT / "a2a_server/forgejo_author_task.py"
POOL_TEST = ROOT / "tests/test_forgejo_work_dedupe.py"
TENANT_TEST = ROOT / "tests/test_forgejo_tenant_binding.py"
POOL_TEST_CONTENT = '''# ruff: noqa: SLF001
"""Forgejo work stays outside the generic GitHub App dedupe path."""

from a2a_server import persistent_worker_pool as pool


def test_forgejo_review_uses_the_dedicated_protocol_boundary():
    metadata = {
        'source': 'forgejo-pr-review',
        'repo': 'owner/repo',
        'pr_number': 7,
        'workflow_stage': 'forgejo-author-review',
        'pr_head_sha': 'abc123',
    }
    assert pool._github_work_key(metadata) is None
'''


def main() -> None:
    """Restore GitHub-only pooling and hash persisted Forgejo markers."""
    text = POOL.read_text()
    replacements = {
        '"""Return a stable idempotency key for active source-control work.':
            '"""Return a stable idempotency key for active GitHub App work.',
        "    if (source := metadata.get('source')) not in ('github-app', 'forgejo-pr-review'):\n":
            "    if metadata.get('source') != 'github-app':\n",
        "    return f'{source}:{repo}:{number}:{stage}:{head_sha}'\n":
            "    return f'github-app:{repo}:{number}:{stage}:{head_sha}'\n",
        '"""Serialize source-control task creation for one work key across API pods."""':
            '"""Serialize GitHub App task creation for one work key across API pods."""',
        '"""Return the oldest active or completed task for an idempotency key."""':
            '"""Return the oldest active task for a GitHub App idempotency key."""',
        '''            WHERE status IN (
                'pending', 'queued', 'running', 'working', 'completed'
            )
              AND COALESCE(metadata, '{}'::jsonb)->>'source' = split_part($1, ':', 1)
''': '''            WHERE status IN ('pending', 'queued', 'running', 'working')
              AND COALESCE(metadata, '{}'::jsonb)->>'source' = 'github-app'
''',
        "                'Reusing source-control task %s for work key %s',\n":
            "                'Reusing active GitHub App task %s for work key %s',\n",
    }
    for old, new in replacements.items():
        text = text.replace(old, new)
    POOL.write_text(text)
    task = TASK.read_text()
    old = '''    metadata['target_worker_id'] = str(worker['worker_id'])
    metadata['idempotency_key'] = key
    metadata['github_work_key'] = key
'''
    new = '''    metadata['target_worker_id'] = str(worker['worker_id'])
    metadata['idempotency_key'] = task_id
    metadata['github_work_key'] = task_id
'''
    if new not in task:
        if task.count(old) != 1:
            raise RuntimeError('hashed idempotency marker anchor is missing')
        TASK.write_text(task.replace(old, new, 1))
    task = TASK.read_text().replace(
        '    key, task_id = task_identity(metadata)\n',
        '    _key, task_id = task_identity(metadata)\n',
    )
    TASK.write_text(task)
    POOL_TEST.write_text(POOL_TEST_CONTENT)
    tenant = TENANT_TEST.read_text()
    old_call = '''    await prepare(value)
    assert seen == ['tenant-a']
'''
    new_call = '''    await prepare(value)
    assert seen == ['tenant-a']
    assert value['idempotency_key'].startswith('cttask_')
    assert 'tenant-a' not in value['idempotency_key']
'''
    if new_call not in tenant:
        if tenant.count(old_call) != 1:
            raise RuntimeError('tenant marker test anchor is missing')
        TENANT_TEST.write_text(tenant.replace(old_call, new_call, 1))


if __name__ == '__main__':
    main()