"""Create signed provenance fixtures and fail-closed HMAC tests."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP/tests')
FIXTURE = ROOT / 'forgejo_provenance_fixture.py'
METADATA = ROOT / 'forgejo_metadata.py'
VERIFY_TEST = ROOT / 'test_forgejo_author_verification.py'
HMAC_TEST = ROOT / 'test_forgejo_provenance_hmac.py'
FIXTURE_CONTENT = '''"""Cryptographically signed CodeTether provenance fixtures."""

import hashlib
import hmac
import json

from a2a_server.forgejo_provenance_payload import canonical

KEY_ID = 'author-key'
SECRET = 'test-provenance-secret'
TENANT = 'tenant'


def registry(value: dict[str, object]) -> str:
    """Return server key configuration bound to the fixture author."""
    binding = {
        'secret': SECRET,
        'agent_identity': value['author_agent_identity'],
            'tenant_id': TENANT,
            'task_auth_label': 'reviewer',
    }
    return json.dumps({KEY_ID: binding})


def signed_message(value: dict[str, object]) -> str:
    """Build a commit message signed with the Rust-compatible HMAC payload."""
    fields = {
        'CodeTether-Provenance-ID': str(value['author_provenance_id']),
        'CodeTether-Session-ID': str(value['resume_session_id']),
        'CodeTether-Tenant-ID': TENANT,
        'CodeTether-Agent-Identity': str(value['author_agent_identity']),
        'CodeTether-Agent-Name': 'author',
        'CodeTether-Origin': 'worker',
        'CodeTether-Key-ID': KEY_ID,
        'CodeTether-Forgejo-Host': str(value['forgejo_host']),
        'CodeTether-Forgejo-Login': str(value['forgejo_author_login']),
        'CodeTether-Agent-Slot': str(value['agent_slot']),
    }
    signature = hmac.new(SECRET.encode(), canonical(fields), hashlib.sha256)
    fields['CodeTether-Signature'] = signature.hexdigest()
    trailers = [f'{label}: {field}' for label, field in fields.items()]
    return '\\n'.join(['Signed author commit', '', *trailers])
'''
HMAC_TEST_CONTENT = '''import json

import pytest

from a2a_server.forgejo_provenance_fields import parse
from a2a_server.forgejo_provenance_verification import verify
from tests.forgejo_metadata import commit_message, metadata
from tests.forgejo_provenance_fixture import registry


def test_fixture_matches_the_rust_provenance_hmac_vector():
    value = metadata()
    assert parse(commit_message(value))['CodeTether-Signature'] == (
        '72bc4744ef5ec3461ed0279c9a0d716d12b87d29e711dd8f6263cfe7149005be'
    )


def test_provenance_hmac_binds_session_agent_and_tenant(monkeypatch):
    value = metadata()
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    key = verify(commit_message(value), value)
    assert key.agent_identity == value['target_agent_name']
    assert key.tenant_id == 'tenant'


@pytest.mark.parametrize('field', ['resume_session_id', 'target_agent_name'])
def test_provenance_hmac_rejects_tampered_metadata(monkeypatch, field):
    signed, forwarded = metadata(), metadata()
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(signed))
    forwarded[field] = 'attacker-value'
    with pytest.raises(ValueError):
        verify(commit_message(signed), forwarded)


def test_provenance_hmac_rejects_unconfigured_keys(monkeypatch):
    monkeypatch.delenv('CODETETHER_PROVENANCE_SIGNING_KEYS', raising=False)
    value = metadata()
    with pytest.raises(RuntimeError, match='not configured'):
        verify(commit_message(value), value)
'''


def main() -> None:
    """Install signed fixtures and verification coverage."""
    FIXTURE.write_text(FIXTURE_CONTENT)
    text = METADATA.read_text()
    if 'from tests.forgejo_provenance_fixture import signed_message\n' not in text:
        text = text.replace(
            'from a2a_server.forgejo_conversation_identity import conversation_id\n',
            'from a2a_server.forgejo_conversation_identity import conversation_id\nfrom tests.forgejo_provenance_fixture import signed_message\n',
        )
    start = text.find('def commit_message(value: dict[str, object]) -> str:\n')
    if start >= 0:
        text = text[:start] + '''def commit_message(value: dict[str, object]) -> str:
    """Build the signed trailer block for a valid envelope."""
    return signed_message(value)
'''
    METADATA.write_text(text)
    verify_test = VERIFY_TEST.read_text()
    if 'from tests.forgejo_provenance_fixture import registry\n' not in verify_test:
        verify_test = verify_test.replace(
            'from tests.forgejo_metadata import metadata\n',
            'from tests.forgejo_metadata import metadata\nfrom tests.forgejo_provenance_fixture import registry\n',
        )
    anchor = "    monkeypatch.setenv('CODETETHER_FORGEJO_API_BASE_URLS', json.dumps(value))\n"
    key_env = "    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(metadata()))\n"
    if key_env not in verify_test:
        verify_test = verify_test.replace(anchor, anchor + key_env, 1)
    VERIFY_TEST.write_text(verify_test)
    HMAC_TEST.write_text(HMAC_TEST_CONTENT)


if __name__ == '__main__':
    main()