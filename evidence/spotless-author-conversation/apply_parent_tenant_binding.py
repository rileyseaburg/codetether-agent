"""Bind Forgejo task identity and routing to authenticated request tenancy."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
REQUEST_SCOPE = ROOT / "a2a_server/forgejo_request_scope.py"
MONITOR = ROOT / "a2a_server/monitor_api.py"
TASK = ROOT / "a2a_server/forgejo_author_task.py"
BRIDGE = ROOT / "a2a_server/agent_bridge.py"
REQUEST_SCOPE_CONTENT = '''"""Authenticated idempotency scope for Forgejo task requests."""

from typing import Optional, Tuple

from fastapi import Request


def resolve(request: Request) -> Tuple[str, Optional[str]]:
    """Return a server-controlled scope and optional database tenant."""
    user = getattr(request.state, 'policy_user', None)
    tenant = _field(user, 'tenant_id')
    subject = _field(user, 'user_id') or _field(user, 'id') or _field(user, 'sub')
    if tenant:
        return f'tenant:{tenant}', tenant
    if subject:
        return f'subject:{subject}', None
    return 'authenticated:global', None


def _field(user: object, key: str) -> str:
    if not isinstance(user, dict):
        return ''
    return str(user.get(key) or '').strip()
'''


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text()
    if new in text:
        return
    if text.count(old) != 1:
        raise RuntimeError(f"tenant binding anchor missing: {path}")
    path.write_text(text.replace(old, new, 1))


def main() -> None:
    """Install server-controlled request scope throughout task persistence."""
    REQUEST_SCOPE.write_text(REQUEST_SCOPE_CONTENT)
    replace_once(
        MONITOR,
        'from .task_orchestration import orchestrate_task_route\n',
        'from .task_orchestration import orchestrate_task_route\nfrom .forgejo_request_scope import resolve as forgejo_request_scope\n',
    )
    replace_once(
        MONITOR,
        '''    return await create_global_task(
        task_data, request.headers.get('x-forgejo-token', '')
    )
''',
        '''    scope, tenant_id = forgejo_request_scope(request)
    return await create_global_task(
        task_data,
        request.headers.get('x-forgejo-token', ''),
        scope,
        tenant_id,
    )
''',
    )
    replace_once(
        MONITOR,
        '''async def create_global_task(
    task_data: AgentTaskCreate, forgejo_token: str = ''
):
''',
        '''async def create_global_task(
    task_data: AgentTaskCreate,
    forgejo_token: str = '',
    idempotency_scope: str = 'internal:global',
    tenant_id: Optional[str] = None,
):
''',
    )
    replace_once(
        MONITOR,
        '''    if routed_metadata.get('protocol') == 'codetether.forgejo-author.v1':
        from .forgejo_author_service import create as create_author_task
''',
        '''    if routed_metadata.get('protocol') == 'codetether.forgejo-author.v1':
        from .forgejo_author_service import create as create_author_task

        routed_metadata['idempotency_scope'] = idempotency_scope
        if tenant_id:
            routed_metadata['tenant_id'] = tenant_id
        else:
            routed_metadata.pop('tenant_id', None)
''',
    )
    replace_once(
        TASK,
        '''        'forgejo-pr-review:v1',
        str(metadata['forgejo_host']).lower(),
''',
        '''        'forgejo-pr-review:v1',
        str(metadata.get('idempotency_scope') or 'internal:global'),
        str(metadata['forgejo_host']).lower(),
''',
    )
    replace_once(
        TASK,
        "    worker = await db.db_get_active_worker_by_name(metadata['target_agent_name'])\n",
        '''    tenant_id = str(metadata.get('tenant_id') or '') or None
    worker = await db.db_get_active_worker_by_name(
        str(metadata['target_agent_name']), tenant_id=tenant_id
    )
''',
    )
    replace_once(
        BRIDGE,
        "                    'metadata': metadata,\n",
        "                    'metadata': metadata,\n                    'tenant_id': metadata.get('tenant_id'),\n",
    )


if __name__ == "__main__":
    main()