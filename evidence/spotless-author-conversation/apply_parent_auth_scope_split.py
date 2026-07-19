"""Separate legacy bearer verification from request principal selection."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/a2a_server")
TOKEN = ROOT / "task_auth_scope.py"
SCOPE = ROOT / "forgejo_request_scope.py"
TOKEN_CONTENT = '''"""Server-controlled scope for configured task bearer tokens."""

import hmac
import os

from fastapi import HTTPException, Request


def legacy_scope(request: Request) -> str:
    """Verify a configured bearer and return its non-secret label."""
    raw = os.environ.get('A2A_AUTH_TOKENS', '')
    if not raw:
        raise HTTPException(
            status_code=503, detail='Task authentication is not configured'
        )
    supplied = _bearer(request)
    for pair in raw.split(','):
        label, separator, expected = pair.strip().partition(':')
        expected = expected.strip()
        if (
            separator
            and label
            and expected
            and hmac.compare_digest(supplied, expected)
        ):
            return f'token:{label}'
    raise HTTPException(status_code=403, detail='Task authentication is invalid')


def _bearer(request: Request) -> str:
    authorization = request.headers.get('authorization', '')
    if not authorization.startswith('Bearer '):
        raise HTTPException(
            status_code=401, detail='Task authentication is required'
        )
    supplied = authorization.removeprefix('Bearer ').strip()
    if not supplied:
        raise HTTPException(
            status_code=401, detail='Task authentication is required'
        )
    return supplied
'''
SCOPE_CONTENT = '''"""Authenticated idempotency scope for Forgejo task requests."""

from fastapi import Request

from a2a_server.task_auth_scope import legacy_scope


def resolve(request: Request) -> tuple[str, str | None]:
    """Return a verified server-controlled scope and optional tenant."""
    user = getattr(request.state, 'policy_user', None)
    tenant = _field(user, 'tenant_id')
    subject = (
        _field(user, 'user_id') or _field(user, 'id') or _field(user, 'sub')
    )
    if tenant:
        return f'tenant:{tenant}', tenant
    if subject:
        return f'subject:{subject}', None
    return legacy_scope(request), None


def _field(user: object, key: str) -> str:
    if not isinstance(user, dict):
        return ''
    return str(user.get(key) or '').strip()
'''


def main() -> None:
    """Write the focused token verifier and request scope resolver."""
    TOKEN.write_text(TOKEN_CONTENT)
    SCOPE.write_text(SCOPE_CONTENT)


if __name__ == '__main__':
    main()