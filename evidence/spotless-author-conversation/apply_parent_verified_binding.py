"""Bind verified provenance tenancy and a server-only resume marker."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
SERVICE = ROOT / 'a2a_server/forgejo_author_service.py'
SERVICE_TEST = ROOT / 'tests/test_forgejo_author_service.py'
CONCURRENCY_TEST = ROOT / 'tests/test_forgejo_author_concurrency.py'


def main() -> None:
    """Install trusted binding metadata after cryptographic verification."""
    text = SERVICE.read_text()
    old = '''    metadata = request.metadata
    await verify(metadata, request.forgejo_token)
    async with serialized(metadata):
'''
    new = '''    metadata = request.metadata
    metadata.pop('server_author_binding_verified', None)
    metadata.pop('author_identity_key_id', None)
    key = await verify(metadata, request.forgejo_token)
    requested_tenant = str(metadata.get('tenant_id') or '')
    if requested_tenant and requested_tenant != key.tenant_id:
        raise ValueError('authenticated tenant does not match author provenance')
    metadata['tenant_id'] = key.tenant_id
    metadata['idempotency_scope'] = f'tenant:{key.tenant_id}'
    metadata['author_identity_key_id'] = key.key_id
    metadata['server_author_binding_verified'] = True
    async with serialized(metadata):
'''
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('verified service binding anchor is missing')
        SERVICE.write_text(text.replace(old, new, 1))
    service_test = SERVICE_TEST.read_text()
    service_test = service_test.replace(
        '''    async def verify(_metadata, token):
        assert token == 'forgejo-token'
        events.append('verify')
''',
        '''    async def verify(_metadata, token):
        assert token == 'forgejo-token'
        events.append('verify')
        return SimpleNamespace(
            key_id='author-key', tenant_id='tenant', agent_identity='target'
        )
''',
    )
    if 'from types import SimpleNamespace\n' not in service_test:
        service_test = 'from types import SimpleNamespace\n\n' + service_test
    assertion = "    assert task['metadata']['server_author_binding_verified'] is True\n"
    if assertion not in service_test:
        anchor = "    assert task['metadata']['tenant_id'] == 'tenant'\n"
        if service_test.count(anchor) != 1:
            raise RuntimeError('verified marker test anchor is missing')
        service_test = service_test.replace(anchor, anchor + assertion, 1)
    SERVICE_TEST.write_text(service_test)
    concurrency = CONCURRENCY_TEST.read_text()
    concurrency = concurrency.replace(
        '''    async def verify(_metadata, _token):
        return None
''',
        '''    async def verify(_metadata, _token):
        return SimpleNamespace(
            key_id='author-key', tenant_id='tenant', agent_identity='target'
        )
''',
    )
    if 'from types import SimpleNamespace\n' not in concurrency:
        concurrency = concurrency.replace(
            'from contextlib import asynccontextmanager\n',
            'from contextlib import asynccontextmanager\nfrom types import SimpleNamespace\n',
        )
    CONCURRENCY_TEST.write_text(concurrency)


if __name__ == '__main__':
    main()