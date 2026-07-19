"""Move the reusable recording bridge out of the service behavior test."""

from pathlib import Path

FIXTURE = Path("/home/riley/A2A-Server-MCP/tests/forgejo_service_fixture.py")
TEST = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_service.py")


def main() -> None:
    """Add the recording bridge fixture and use it from the focused test."""
    fixture = FIXTURE.read_text()
    bridge = '''

class RecordingBridge:
    """Task bridge that records and returns creation arguments."""

    def __init__(self, events: list[str]) -> None:
        self.events = events

    async def create_task(self, **kwargs: object) -> dict[str, object]:
        self.events.append('create')
        return kwargs
'''
    if 'class RecordingBridge' not in fixture:
        FIXTURE.write_text(fixture.rstrip() + bridge)
    test = TEST.read_text()
    test = test.replace(
        'from tests.forgejo_service_fixture import request\n',
        'from tests.forgejo_service_fixture import RecordingBridge, request\n',
    )
    start = test.find('    class Bridge:\n')
    end = test.find('    monkeypatch.setattr', start)
    if start >= 0 and end >= 0:
        test = test[:start] + test[end:]
    test = test.replace(
        'service.create(Bridge(),',
        'service.create(RecordingBridge(events),',
    )
    test = test.replace(
        '        Bridge(),\n',
        '        RecordingBridge(events),\n',
    )
    TEST.write_text(test)


if __name__ == '__main__':
    main()