"""Bind every Forgejo author field to independently fetched signed trailers."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
TRAILERS = ROOT / "a2a_server/forgejo_commit_trailers.py"
VERIFY = ROOT / "a2a_server/forgejo_author_verification.py"
FIXTURE = ROOT / "tests/forgejo_metadata.py"
VERIFY_TEST = ROOT / "tests/test_forgejo_author_verification.py"
TRAILER_TEST = ROOT / "tests/test_forgejo_commit_trailers.py"
TRANSPORT = ROOT / "tests/forgejo_verification_transport.py"
TRAILER_CONTENT = '''"""Required CodeTether trailers from a Forgejo-signed commit."""

from typing import Mapping

FIELDS = {
    'CodeTether-Agent-Identity': 'author_agent_identity',
    'CodeTether-Session-ID': 'resume_session_id',
    'CodeTether-Provenance-ID': 'author_provenance_id',
    'CodeTether-Forgejo-Host': 'forgejo_host',
    'CodeTether-Forgejo-Login': 'forgejo_author_login',
    'CodeTether-Agent-Slot': 'agent_slot',
}


def verify_binding(metadata: Mapping[str, object], message: str) -> None:
    """Require each envelope field to equal one unique signed trailer."""
    trailers = parse(message)
    for label, field in FIELDS.items():
        expected = str(metadata.get(field) or '')
        if trailers[label].lower() != expected.lower():
            raise ValueError(f'signed trailer does not match {field}')
    if trailers['CodeTether-Agent-Identity'] != metadata.get('target_agent_name'):
        raise ValueError('signed agent identity does not match target')


def parse(message: str) -> dict[str, str]:
    """Parse exactly one nonempty value for every required trailer."""
    found: dict[str, list[str]] = {label: [] for label in FIELDS}
    for line in message.splitlines():
        label, separator, value = line.partition(':')
        if separator and label in found and value.strip():
            found[label].append(value.strip())
    if any(len(values) != 1 for values in found.values()):
        raise ValueError('signed CodeTether trailers are missing or ambiguous')
    return {label: values[0] for label, values in found.items()}
'''
TEST_CONTENT = '''import pytest

from a2a_server.forgejo_commit_trailers import parse, verify_binding
from tests.forgejo_metadata import commit_message, metadata


def test_all_protocol_fields_are_bound_to_signed_trailers():
    value = metadata()
    verify_binding(value, commit_message(value))
    parsed = parse(commit_message(value))
    assert parsed['CodeTether-Agent-Identity'] == value['target_agent_name']


def test_changed_provenance_is_rejected():
    value = metadata()
    message = commit_message(value)
    value['author_provenance_id'] = 'ctprov_abcdef1234567890'
    with pytest.raises(ValueError, match='author_provenance_id'):
        verify_binding(value, message)


def test_duplicate_signed_trailer_is_rejected():
    value = metadata()
    message = commit_message(value) + '\\nCodeTether-Agent-Slot: other\\n'
    with pytest.raises(ValueError, match='ambiguous'):
        parse(message)
'''
TRANSPORT_CONTENT = '''"""Mocked Forgejo proof transport for verification tests."""

import httpx

from tests.forgejo_metadata import commit_message, metadata


def transport(*, signer='alice', head=None, state='open', signed=None):
    """Return PR and commit proof for one signed metadata envelope."""
    expected = 'a' * 40
    signed = signed or metadata()

    def respond(request):
        assert request.headers['authorization'] == 'token scoped-token'
        if '/pulls/' in request.url.path:
            value = {
                'state': state,
                'head': {'sha': head or expected},
                'user': {'login': 'alice'},
            }
        else:
            value = {
                'sha': expected,
                'commit': {
                    'message': commit_message(signed),
                    'verification': {
                        'verified': True,
                        'signer': {'username': signer},
                    },
                },
            }
        return httpx.Response(200, json=value)

    return httpx.MockTransport(respond)
'''
VERIFY_TEST_CONTENT = '''import json

import pytest

from a2a_server.forgejo_author_verification import verify
from tests.forgejo_metadata import metadata
from tests.forgejo_verification_transport import transport


@pytest.fixture(autouse=True)
def configured_host(monkeypatch):
    value = {'forge.example': 'https://forge.example/api/v1'}
    monkeypatch.setenv('CODETETHER_FORGEJO_API_BASE_URLS', json.dumps(value))


@pytest.mark.asyncio
async def test_server_independently_verifies_pr_signer_and_trailers():
    await verify(metadata(), 'scoped-token', transport())


@pytest.mark.asyncio
@pytest.mark.parametrize('change', ['signer', 'head', 'state'])
async def test_server_rejects_stale_or_unverified_proof(change):
    kwargs = {change: {'signer': 'mallory', 'head': 'b' * 40, 'state': 'closed'}[change]}
    with pytest.raises((ValueError, LookupError)):
        await verify(metadata(), 'scoped-token', transport(**kwargs))


@pytest.mark.asyncio
async def test_server_rejects_metadata_not_present_in_signed_commit():
    signed, forwarded = metadata(), metadata()
    forwarded['author_provenance_id'] = 'ctprov_abcdef1234567890'
    with pytest.raises(ValueError, match='author_provenance_id'):
        await verify(forwarded, 'scoped-token', transport(signed=signed))


@pytest.mark.asyncio
async def test_server_requires_a_non_persisted_verification_token():
    with pytest.raises(RuntimeError, match='credential'):
        await verify(metadata(), '', transport())
'''


def main() -> None:
    """Write trailer verification and connect it to Forgejo responses."""
    TRAILERS.write_text(TRAILER_CONTENT)
    TRAILER_TEST.write_text(TEST_CONTENT)
    TRANSPORT.write_text(TRANSPORT_CONTENT)
    VERIFY_TEST.write_text(VERIFY_TEST_CONTENT)
    text = VERIFY.read_text()
    import_line = 'from .forgejo_commit_trailers import verify_binding\n'
    if import_line not in text:
        anchor = 'from .forgejo_author_contract import validate\n'
        if text.count(anchor) != 1:
            raise RuntimeError('signed trailer import anchor is missing')
        text = text.replace(anchor, anchor + import_line, 1)
    anchor = "    signer = nested(commit, 'commit', 'verification', 'signer', 'username')\n"
    addition = '''    message = nested(commit, 'commit', 'message')
    if not isinstance(message, str):
        raise ValueError('Forgejo commit message is missing')
    verify_binding(metadata, message)
'''
    if addition not in text:
        if text.count(anchor) != 1:
            raise RuntimeError('signed commit response anchor is missing')
        text = text.replace(anchor, addition + anchor, 1)
    VERIFY.write_text(text)
    fixture = FIXTURE.read_text()
    helper = '''

def commit_message(value: dict[str, object]) -> str:
    """Build the exact signed trailer block for a valid envelope."""
    return '\\n'.join([
        'Signed author commit', '',
        f"CodeTether-Agent-Identity: {value['author_agent_identity']}",
        f"CodeTether-Session-ID: {value['resume_session_id']}",
        f"CodeTether-Provenance-ID: {value['author_provenance_id']}",
        f"CodeTether-Forgejo-Host: {value['forgejo_host']}",
        f"CodeTether-Forgejo-Login: {value['forgejo_author_login']}",
        f"CodeTether-Agent-Slot: {value['agent_slot']}",
    ])
'''
    if 'def commit_message(' not in fixture:
        FIXTURE.write_text(fixture.rstrip() + helper)


if __name__ == "__main__":
    main()