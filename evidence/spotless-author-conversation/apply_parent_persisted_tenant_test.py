"""Assert authenticated tenancy reaches the durable task record."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_agent_bridge_save_result.py")


def main() -> None:
    """Capture the failed upsert record and assert tenant propagation."""
    text = PATH.read_text()
    text = text.replace(
        '''async def test_save_task_propagates_database_false(monkeypatch):
    async def failed_upsert(_task):
        return False
''',
        '''async def test_save_task_propagates_database_false(monkeypatch):
    records = []

    async def failed_upsert(task):
        records.append(task)
        return False
''',
    )
    text = text.replace("        metadata={},\n", "        metadata={'tenant_id': 'tenant-a'},\n")
    assertion = "    assert records[0]['tenant_id'] == 'tenant-a'\n"
    if assertion not in text:
        anchor = (
            '    assert await AgentBridge.__new__(AgentBridge)._save_task(task) '
            'is False\n'
        )
        if text.count(anchor) != 1:
            raise RuntimeError('tenant persistence assertion anchor is missing')
        text = text.replace(anchor, anchor + assertion, 1)
    PATH.write_text(text)


if __name__ == '__main__':
    main()