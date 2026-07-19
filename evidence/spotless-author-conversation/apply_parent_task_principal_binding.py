"""Bind task bearer principals to provenance keys and tenants."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
MODULE = ROOT / 'a2a_server/forgejo_task_authorization.py'
AUTHENTICATE = ROOT / 'a2a_server/forgejo_author_authenticate.py'
SERVICE = ROOT / 'a2a_server/forgejo_author_service.py'
FIXTURE = ROOT / 'tests/forgejo_provenance_fixture.py'
SERVICE_FIXTURE = ROOT / 'tests/forgejo_service_fixture.py'
TEST = ROOT / 'tests/test_forgejo_task_authorization.py'
REQUEST = ROOT / 'a2a_server/forgejo_author_request.py'
MONITOR = ROOT / 'a2a_server/monitor_api.py'
MODULE_CONTENT = '''"""Task-principal authorization for a verified provenance key."""

from a2a_server.forgejo_provenance_keys import ProvenanceKey


def require(key: ProvenanceKey, scope: str, tenant_id: str | None) -> None:
    """Require the task credential to belong to the key's trusted principal."""
    if tenant_id is not None:
        if tenant_id != key.tenant_id:
            raise ValueError('task credential tenant does not match provenance key')
        return
    if key.task_auth_label is not None:
        if not scope.startswith(f'token:{key.task_auth_label}:'):
            raise ValueError('task credential principal does not match provenance key')
        return
    if tenant_id is None:
        raise RuntimeError('provenance key lacks a task principal binding')
'''
AUTHENTICATE_CONTENT = '''"""Authentication boundary for Forgejo author task creation."""

from collections.abc import MutableMapping

from a2a_server.forgejo_author_binding import apply
from a2a_server.forgejo_author_request import AuthorTaskRequest
from a2a_server.forgejo_author_verification import verify
from a2a_server.forgejo_task_authorization import require


async def authenticate(request: AuthorTaskRequest) -> MutableMapping[str, object]:
    """Return metadata bound to independently verified principals."""
    metadata = request.metadata
    metadata.pop('server_author_binding_verified', None)
    metadata.pop('author_identity_key_id', None)
    key = await verify(metadata, request.forgejo_token)
    require(key, request.idempotency_scope, request.tenant_id)
    apply(metadata, key)
    return metadata
'''
TEST_CONTENT = '''import pytest

from a2a_server.forgejo_task_authorization import require
from tests.forgejo_service_fixture import provenance_key


def test_author_key_accepts_only_its_configured_task_principal():
    require(provenance_key(), 'token:reviewer:credential-hash', None)
    with pytest.raises(ValueError, match='principal'):
        require(provenance_key(), 'token:attacker:credential-hash', None)


def test_policy_tenant_must_match_the_author_key():
    require(provenance_key(), 'tenant:tenant', 'tenant')
    with pytest.raises(ValueError, match='tenant'):
        require(provenance_key(), 'tenant:other', 'other')
'''


def main() -> None:
    """Install task-principal authorization before author task preparation."""
    MODULE.write_text(MODULE_CONTENT)
    AUTHENTICATE.write_text(AUTHENTICATE_CONTENT)
    service = SERVICE.read_text()
    import_line = 'from a2a_server.forgejo_author_authenticate import authenticate\n'
    obsolete = (
        'from a2a_server.forgejo_author_binding import apply as apply_binding\n',
        'from a2a_server.forgejo_author_verification import verify\n',
        'from a2a_server.forgejo_task_authorization import (\n'
        '    require as authorize_task_principal,\n)\n',
    )
    for value in obsolete:
        service = service.replace(value, '')
    if import_line not in service:
        anchor = 'from a2a_server.forgejo_author_lock import serialized\n'
        service = service.replace(anchor, import_line + anchor, 1)
    start = "    metadata = request.metadata\n"
    end = "    apply_binding(metadata, key)\n"
    if end in service:
        before, remainder = service.split(start, 1)
        _removed, after = remainder.split(end, 1)
        service = before + '    metadata = await authenticate(request)\n' + after
    SERVICE.write_text(service)
    fixture = FIXTURE.read_text()
    old = "        'tenant_id': TENANT,\n"
    new = old + "        'task_auth_label': 'reviewer',\n"
    if "        'task_auth_label': 'reviewer',\n" not in fixture:
        fixture = fixture.replace(old, new, 1)
    FIXTURE.write_text(fixture)
    service_fixture = SERVICE_FIXTURE.read_text()
    request_source = REQUEST.read_text()
    field_anchor = '    forgejo_token: str\n'
    fields = field_anchor + '    idempotency_scope: str\n    tenant_id: str | None\n'
    if '    idempotency_scope: str\n' not in request_source:
        REQUEST.write_text(request_source.replace(field_anchor, fields, 1))
    monitor = MONITOR.read_text()
    constructor_anchor = '''                workspace_id=effective_workspace_id,
                forgejo_token=forgejo_token,
'''
    constructor_fields = constructor_anchor + '''                idempotency_scope=idempotency_scope,
                tenant_id=tenant_id,
'''
    if '                idempotency_scope=idempotency_scope,\n' not in monitor:
        MONITOR.write_text(monitor.replace(constructor_anchor, constructor_fields, 1))
    old_request = '''    return AuthorTaskRequest(
        task_data=SimpleNamespace(
'''
    new_request = '''    values = {
        'idempotency_scope': 'token:reviewer:fingerprint',
        'tenant_id': 'tenant',
    }
    values.update(metadata or {})
    return AuthorTaskRequest(
        task_data=SimpleNamespace(
'''
    if new_request not in service_fixture:
        service_fixture = service_fixture.replace(old_request, new_request, 1)
        service_fixture = service_fixture.replace('        metadata=metadata or {},\n', '        metadata=values,\n', 1)
    if "task_auth_label='reviewer'" not in service_fixture:
        service_fixture = service_fixture.replace(
            "key_id='author-key', tenant_id='tenant', agent_identity='target'",
            "key_id='author-key', tenant_id='tenant', "
            "agent_identity='target', task_auth_label='reviewer'",
            1,
        )
    SERVICE_FIXTURE.write_text(service_fixture)
    service_fixture = SERVICE_FIXTURE.read_text()
    request_anchor = "        forgejo_token=token,\n"
    request_fields = (
        request_anchor
        + "        idempotency_scope=str(values['idempotency_scope']),\n"
        + "        tenant_id=str(values['tenant_id']),\n"
    )
    if '        idempotency_scope=str(values' not in service_fixture:
        SERVICE_FIXTURE.write_text(service_fixture.replace(request_anchor, request_fields, 1))
    service_test = ROOT / 'tests/test_forgejo_author_service.py'
    service_text = service_test.read_text().replace(
        "'idempotency_scope': 'token:reviewer',",
        "'idempotency_scope': 'token:reviewer:fingerprint',",
    )
    service_test.write_text(service_text)
    concurrency_test = ROOT / 'tests/test_forgejo_author_concurrency.py'
    concurrency = concurrency_test.read_text()
    auth_import = 'import a2a_server.forgejo_author_authenticate as authentication\n'
    if auth_import not in concurrency:
        anchor = 'import a2a_server.forgejo_author_service as service\n'
        concurrency = concurrency.replace(anchor, auth_import + anchor, 1)
    concurrency = concurrency.replace(
        "monkeypatch.setattr(service, 'verify', verify)",
        "monkeypatch.setattr(authentication, 'verify', verify)",
    )
    concurrency_test.write_text(concurrency)
    TEST.write_text(TEST_CONTENT)


if __name__ == '__main__':
    main()