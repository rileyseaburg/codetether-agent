"""Preserve the resolved model route on deterministic author tasks."""

from pathlib import Path

SERVICE = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_service.py")
TYPES = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_types.py")
TEST = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_service.py")


def main() -> None:
    """Pass model fields to the bridge without mutating HTTP input data."""
    text = SERVICE.read_text()
    old = '''        task_data = request.task_data
        task_data.metadata = metadata
        task_data.routing = request.routing
        result = await bridge.create_task(
'''
    new = '''        task_data = request.task_data
        model = str(metadata['model']) if metadata.get('model') else None
        result = await bridge.create_task(
'''
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('author model route service anchor is missing')
        text = text.replace(old, new, 1)
    old_call = '''            priority=task_data.priority,
            metadata=metadata,
'''
    new_call = '''            priority=task_data.priority,
            model=model,
            model_ref=request.routing.model_ref,
            metadata=metadata,
'''
    if new_call not in text:
        if text.count(old_call) != 1:
            raise RuntimeError('author bridge model anchor is missing')
        text = text.replace(old_call, new_call, 1)
    SERVICE.write_text(text)
    types = TYPES.read_text()
    types = types.replace(
        '''    metadata: MutableMapping[str, object]
    routing: RoutingDecision
''',
        '',
    )
    TYPES.write_text(types)
    test = TEST.read_text()
    assertion = "    assert task['model'] == 'test'\n"
    if assertion not in test:
        anchor = "    assert task['task_id'] == 'cttask_fixed'\n"
        if test.count(anchor) != 1:
            raise RuntimeError('author model route test anchor is missing')
        TEST.write_text(test.replace(anchor, anchor + assertion, 1))


if __name__ == '__main__':
    main()