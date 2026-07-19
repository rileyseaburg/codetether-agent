"""Install independent Forgejo verification at the external server boundary."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/a2a_server")
CONFIG = ROOT / "forgejo_verification_config.py"
VERIFY = ROOT / "forgejo_author_verification.py"
SERVICE = ROOT / "forgejo_author_service.py"
MONITOR = ROOT / "monitor_api.py"
CONFIG_CONTENT = '''"""Allowlisted Forgejo API endpoint resolution."""

import json
import os
from urllib.parse import urlparse


def api_base(host: str) -> str:
    """Resolve a Forgejo host only through explicit server configuration."""
    configured = _configured_hosts()
    value = configured.get(host.lower())
    if not isinstance(value, str) or not value:
        raise RuntimeError('Forgejo host is not configured for verification')
    parsed = urlparse(value)
    if parsed.scheme != 'https' or parsed.hostname != host.lower():
        raise RuntimeError('Forgejo verification endpoint is unsafe')
    return value.rstrip('/')


def _configured_hosts() -> dict[str, object]:
    raw = os.environ.get('CODETETHER_FORGEJO_API_BASE_URLS', '')
    if raw:
        try:
            value = json.loads(raw)
        except json.JSONDecodeError as error:
            raise RuntimeError('Forgejo host configuration is invalid') from error
        if isinstance(value, dict):
            return value
        raise RuntimeError('Forgejo host configuration must be an object')
    single = os.environ.get('CODETETHER_FORGEJO_API_BASE_URL', '')
    parsed = urlparse(single)
    return {parsed.hostname: single} if parsed.hostname else {}
'''
VERIFY_CONTENT = '''"""Independent Forgejo proof for author conversation requests."""

from typing import Mapping, Optional

import httpx

from .forgejo_author_identity import validate
from .forgejo_verification_config import api_base


async def verify(
    metadata: Mapping[str, object],
    token: str,
    transport: Optional[httpx.AsyncBaseTransport] = None,
) -> None:
    """Verify the current PR head and signed commit against Forgejo."""
    validate(metadata)
    if not token:
        raise RuntimeError('Forgejo verification credential is required')
    host = str(metadata.get('forgejo_host') or '').lower()
    repo = str(metadata.get('repo') or '')
    number = str(metadata.get('pr_number') or '')
    head = str(metadata.get('pr_head_sha') or '').lower()
    login = str(metadata.get('forgejo_author_login') or '').lower()
    headers = {'Authorization': f'token {token}', 'Accept': 'application/json'}
    try:
        async with httpx.AsyncClient(
            timeout=15.0, transport=transport, headers=headers
        ) as client:
            pull = await _json(client, f'{api_base(host)}/repos/{repo}/pulls/{number}')
            commit = await _json(client, f'{api_base(host)}/repos/{repo}/git/commits/{head}')
    except httpx.HTTPError as error:
        raise RuntimeError('Forgejo verification request failed') from error
    if pull.get('state') != 'open' or _nested(pull, 'head', 'sha') != head:
        raise LookupError('Forgejo PR head is no longer current')
    if str(_nested(pull, 'user', 'login')).lower() != login:
        raise ValueError('Forgejo PR author does not match signed principal')
    signer = _nested(commit, 'commit', 'verification', 'signer', 'username')
    verified = _nested(commit, 'commit', 'verification', 'verified')
    if commit.get('sha') != head or verified is not True or str(signer).lower() != login:
        raise ValueError('Forgejo did not verify the author commit signature')


async def _json(client: httpx.AsyncClient, url: str) -> dict[str, object]:
    response = await client.get(url)
    response.raise_for_status()
    value = response.json()
    if not isinstance(value, dict):
        raise ValueError('Forgejo verification response is invalid')
    return value


def _nested(value: object, *keys: str) -> object:
    for key in keys:
        if not isinstance(value, dict):
            return None
        value = value.get(key)
    return value
'''


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text()
    if new in text:
        return
    if text.count(old) != 1:
        raise RuntimeError(f"Forgejo verification anchor missing: {path}")
    path.write_text(text.replace(old, new, 1))


def main() -> None:
    """Write verification modules and wire token flow without persistence."""
    CONFIG.write_text(CONFIG_CONTENT)
    VERIFY.write_text(VERIFY_CONTENT)
    replace_once(
        SERVICE,
        'from .forgejo_author_task import prepare\n',
        'from .forgejo_author_task import prepare\nfrom .forgejo_author_verification import verify\n',
    )
    replace_once(
        SERVICE,
        '''    workspace_id: str,
    validate_worker: WorkerValidator,
) -> object:
    """Create or replay exactly one durable task for an immutable PR head."""
    async with serialized(metadata):
''',
        '''    workspace_id: str,
    validate_worker: WorkerValidator,
    forgejo_token: str,
) -> object:
    """Create or replay exactly one durable task for an immutable PR head."""
    await verify(metadata, forgejo_token)
    async with serialized(metadata):
''',
    )
    replace_once(
        MONITOR,
        'async def create_global_task(task_data: AgentTaskCreate):\n',
        'async def create_global_task(task_data: AgentTaskCreate, request: Request):\n',
    )
    replace_once(
        MONITOR,
        '''                effective_workspace_id,
                _validate_target_worker_is_available,
            )
''',
        '''                effective_workspace_id,
                _validate_target_worker_is_available,
                request.headers.get('x-forgejo-token', ''),
            )
''',
    )


if __name__ == "__main__":
    main()