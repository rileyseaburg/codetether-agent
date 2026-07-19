"""Require authenticated server-side scope for Forgejo author tasks only."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
SCOPE = ROOT / "a2a_server/forgejo_request_scope.py"
MONITOR = ROOT / "a2a_server/monitor_api.py"
TEST = ROOT / "tests/test_forgejo_request_scope.py"
SCOPE_CONTENT = '''"""Authenticated idempotency scope for Forgejo task requests."""

import hmac
import os
from typing import Optional, Tuple

from fastapi import HTTPException, Request


def resolve(request: Request) -> Tuple[str, Optional[str]]:
    """Return a verified server-controlled scope and optional tenant."""
    user = getattr(request.state, 'policy_user', None)
    tenant = _field(user, 'tenant_id')
    subject = _field(user, 'user_id') or _field(user, 'id') or _field(user, 'sub')
    if tenant:
        return f'tenant:{tenant}', tenant
    if subject:
        return f'subject:{subject}', None
    return _legacy_token_scope(request), None


def _legacy_token_scope(request: Request) -> str:
    raw = os.environ.get('A2A_AUTH_TOKENS', '')
    if not raw:
        raise HTTPException(status_code=503, detail='Task authentication is not configured')
    authorization = request.headers.get('authorization', '')
    if not authorization.startswith('Bearer '):
        raise HTTPException(status_code=401, detail='Task authentication is required')
    supplied = authorization.removeprefix('Bearer ').strip()
    if not supplied:
        raise HTTPException(status_code=401, detail='Task authentication is required')
    for pair in raw.split(','):
        label, separator, expected = pair.strip().partition(':')
        expected = expected.strip()
        if separator and label and expected and hmac.compare_digest(supplied, expected):
            return f'token:{label}'
    raise HTTPException(status_code=403, detail='Task authentication is invalid')


def _field(user: object, key: str) -> str:
    if not isinstance(user, dict):
        return ''
    return str(user.get(key) or '').strip()
'''
TEST_CONTENT = '''from types import SimpleNamespace

import pytest
from fastapi import HTTPException

from a2a_server.forgejo_request_scope import resolve


def request(token=''):
    headers = {'authorization': f'Bearer {token}'} if token else {}
    return SimpleNamespace(state=SimpleNamespace(), headers=headers)


def test_configured_token_label_scopes_the_request(monkeypatch):
    monkeypatch.setenv('A2A_AUTH_TOKENS', 'reviewer:secret,other:different')
    assert resolve(request('secret')) == ('token:reviewer', None)


@pytest.mark.parametrize('token,status', [('', 401), ('wrong', 403)])
def test_missing_or_invalid_task_token_fails_closed(monkeypatch, token, status):
    monkeypatch.setenv('A2A_AUTH_TOKENS', 'reviewer:secret')
    with pytest.raises(HTTPException) as error:
        resolve(request(token))
    assert error.value.status_code == status


def test_unconfigured_task_authentication_fails_closed(monkeypatch):
    monkeypatch.delenv('A2A_AUTH_TOKENS', raising=False)
    with pytest.raises(HTTPException) as error:
        resolve(request('secret'))
    assert error.value.status_code == 503
'''


def main() -> None:
    """Install scoped bearer verification and branch-only invocation."""
    SCOPE.write_text(SCOPE_CONTENT)
    TEST.write_text(TEST_CONTENT)
    text = MONITOR.read_text()
    old = '''    scope, tenant_id = forgejo_request_scope(request)
    return await create_global_task(
'''
    new = '''    scope, tenant_id = 'internal:global', None
    metadata = task_data.metadata or {}
    if metadata.get('protocol') == 'codetether.forgejo-author.v1':
        scope, tenant_id = forgejo_request_scope(request)
    return await create_global_task(
'''
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('protocol authentication wrapper anchor is missing')
        MONITOR.write_text(text.replace(old, new, 1))


if __name__ == "__main__":
    main()