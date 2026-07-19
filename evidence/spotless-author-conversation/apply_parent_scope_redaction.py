"""Keep authenticated replay labels out of persisted agent metadata."""

from pathlib import Path

SERVICE = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_service.py")
TEST = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_service.py")


def main() -> None:
    """Redact only the internal scope after deterministic lookup."""
    text = SERVICE.read_text()
    old = '''        await validate_worker(metadata, strict=True)
        task_data = request.task_data
        task_data.metadata = metadata
'''
    new = '''        await validate_worker(metadata, strict=True)
        metadata = dict(metadata)
        metadata.pop('idempotency_scope', None)
        task_data = request.task_data
        task_data.metadata = metadata
'''
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('scope redaction service anchor is missing')
        SERVICE.write_text(text.replace(old, new, 1))
    test = TEST.read_text()
    test = test.replace(
        "request({'model': 'test'}, 'forgejo-token')",
        "request({'model': 'test', 'idempotency_scope': 'token:reviewer', 'tenant_id': 'tenant'}, 'forgejo-token')",
    )
    assertion = '''    assert 'idempotency_scope' not in task['metadata']
    assert task['metadata']['tenant_id'] == 'tenant'
'''
    if assertion not in test:
        anchor = "    assert 'forgejo-token' not in str(task)\n"
        if test.count(anchor) != 1:
            raise RuntimeError('scope redaction test anchor is missing')
        test = test.replace(anchor, anchor + assertion, 1)
        TEST.write_text(test)


if __name__ == '__main__':
    main()