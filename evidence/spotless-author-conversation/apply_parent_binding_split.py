"""Extract trusted author binding and its shared test fixture."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
BINDING = ROOT / 'a2a_server/forgejo_author_binding.py'
SERVICE = ROOT / 'a2a_server/forgejo_author_service.py'
FIXTURE = ROOT / 'tests/forgejo_service_fixture.py'
SERVICE_TEST = ROOT / 'tests/test_forgejo_author_service.py'
CONCURRENCY_TEST = ROOT / 'tests/test_forgejo_author_concurrency.py'
BINDING_CONTENT = '''"""Server-controlled tenancy for a verified author binding."""

from collections.abc import MutableMapping

from a2a_server.forgejo_provenance_keys import ProvenanceKey


def apply(metadata: MutableMapping[str, object], key: ProvenanceKey) -> None:
    """Replace client aliases with the key's trusted tenant and identity."""
    requested_tenant = str(metadata.get('tenant_id') or '')
    if requested_tenant and requested_tenant != key.tenant_id:
        raise ValueError('authenticated tenant does not match author provenance')
    metadata['tenant_id'] = key.tenant_id
    metadata['idempotency_scope'] = f'tenant:{key.tenant_id}'
    metadata['author_identity_key_id'] = key.key_id
    metadata['server_author_binding_verified'] = True
'''


def main() -> None:
    """Write the binding module and reduce service/test responsibilities."""
    BINDING.write_text(BINDING_CONTENT)
    text = SERVICE.read_text()
    start = text.find("    requested_tenant = str(metadata.get('tenant_id') or '')\n")
    end = text.find('    async with serialized(metadata):\n', start)
    if start >= 0 and end >= 0:
        text = (
            text[:start]
            + '    apply_binding(metadata, key)\n'
            + text[end:]
        )
    binding_import = 'from a2a_server.forgejo_author_binding import apply as apply_binding\n'
    if binding_import not in text:
        text = text.replace(
            'from a2a_server.forgejo_author_lock import serialized\n',
            binding_import + 'from a2a_server.forgejo_author_lock import serialized\n',
        )
    old = '''    requested_tenant = str(metadata.get('tenant_id') or '')
    if requested_tenant and requested_tenant != key.tenant_id:
        raise ValueError('authenticated tenant does not match author provenance')
    metadata['tenant_id'] = key.tenant_id
    metadata['idempotency_scope'] = f'tenant:{key.tenant_id}'
    metadata['author_identity_key_id'] = key.key_id
    metadata['server_author_binding_verified'] = True
'''
    text = text.replace(old, '    apply_binding(metadata, key)\n')
    SERVICE.write_text(text)
    fixture = FIXTURE.read_text()
    if 'def provenance_key()' not in fixture:
        fixture = fixture.rstrip() + '''


def provenance_key() -> SimpleNamespace:
    """Return one trusted author key binding."""
    return SimpleNamespace(
        key_id='author-key', tenant_id='tenant', agent_identity='target'
    )
'''
    FIXTURE.write_text(fixture)
    service_test = SERVICE_TEST.read_text().replace(
        'from types import SimpleNamespace\n\n', ''
    )
    service_test = service_test.replace(
        'from tests.forgejo_service_fixture import RecordingBridge, request\n',
        'from tests.forgejo_service_fixture import RecordingBridge, provenance_key, request\n',
    )
    service_test = service_test.replace(
        '''        return SimpleNamespace(
            key_id='author-key', tenant_id='tenant', agent_identity='target'
        )
''',
        '        return provenance_key()\n',
    )
    SERVICE_TEST.write_text(service_test)
    concurrency = CONCURRENCY_TEST.read_text().replace(
        'from types import SimpleNamespace\n', ''
    )
    concurrency = concurrency.replace(
        'from tests.forgejo_service_fixture import request\n',
        'from tests.forgejo_service_fixture import provenance_key, request\n',
    )
    concurrency = concurrency.replace(
        '''        return SimpleNamespace(
            key_id='author-key', tenant_id='tenant', agent_identity='target'
        )
''',
        '        return provenance_key()\n',
    )
    CONCURRENCY_TEST.write_text(concurrency)


if __name__ == '__main__':
    main()