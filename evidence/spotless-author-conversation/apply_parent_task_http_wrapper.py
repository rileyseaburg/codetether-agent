"""Separate Forgejo token HTTP parsing from the reusable task creator."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/monitor_api.py")
OLD = '''@agent_router_alias.post('/tasks')
async def create_global_task(task_data: AgentTaskCreate, request: Request):
    """Create a new task, optionally tied to a specific workspace.
'''
NEW = '''@agent_router_alias.post('/tasks')
async def create_global_task_endpoint(
    task_data: AgentTaskCreate, request: Request
):
    """Parse HTTP-only verification credentials before task creation."""
    return await create_global_task(
        task_data, request.headers.get('x-forgejo-token', '')
    )


async def create_global_task(
    task_data: AgentTaskCreate, forgejo_token: str = ''
):
    """Create a new task, optionally tied to a specific workspace.
'''
TOKEN_OLD = "                request.headers.get('x-forgejo-token', ''),\n"
TOKEN_NEW = "                forgejo_token,\n"


def main() -> None:
    """Install the wrapper and retain direct-call compatibility."""
    text = PATH.read_text()
    if NEW not in text:
        if text.count(OLD) != 1:
            raise RuntimeError('task HTTP wrapper anchor is missing')
        text = text.replace(OLD, NEW, 1)
    if TOKEN_OLD in text:
        text = text.replace(TOKEN_OLD, TOKEN_NEW, 1)
    if TOKEN_NEW not in text:
        raise RuntimeError('Forgejo token service argument is missing')
    PATH.write_text(text)


if __name__ == "__main__":
    main()