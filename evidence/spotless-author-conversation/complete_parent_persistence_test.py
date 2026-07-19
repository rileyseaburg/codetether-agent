"""Complete the externally patched required-persistence test fixture."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_agent_bridge_required_persistence.py")
TRUNCATED = "\n@pytest.mark.asyncio\n"
COMPLETE = """
@pytest.mark.asyncio
async def test_non_required_cache_remains_backward_compatible():
    bridge = bridge_with_save_result(False)
    cached = object()
    bridge._tasks['cttask_fixed'] = cached
    task = await bridge.create_task(
        codebase_id=None, title='legacy', prompt='legacy', task_id='cttask_fixed'
    )
    assert task is cached
"""


def main() -> None:
    """Replace the single truncated decorator with its complete test."""
    text = PATH.read_text()
    if COMPLETE.strip() in text:
        return
    if not text.endswith(TRUNCATED):
        raise RuntimeError("persistence test is not in the expected truncated state")
    PATH.write_text(text.removesuffix(TRUNCATED) + "\n" + COMPLETE.lstrip())


if __name__ == "__main__":
    main()