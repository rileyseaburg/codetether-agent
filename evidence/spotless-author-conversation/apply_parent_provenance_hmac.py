"""Install server-side verification for CodeTether provenance HMACs."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP/a2a_server')


def write(name: str, content: str) -> None:
    """Write one focused protocol module."""
    (ROOT / name).write_text(content)


def main() -> None:
    """Create typed key, trailer, and HMAC verification modules."""
    write('forgejo_provenance_fields.py', '''"""Signed fields in a CodeTether provenance envelope."""

REQUIRED = (
    'CodeTether-Provenance-ID',
    'CodeTether-Session-ID',
    'CodeTether-Tenant-ID',
    'CodeTether-Agent-Identity',
    'CodeTether-Agent-Name',
    'CodeTether-Origin',
    'CodeTether-Key-ID',
    'CodeTether-Signature',
)
OPTIONAL = (
    'CodeTether-Task-ID',
    'CodeTether-Run-ID',
    'CodeTether-Attempt-ID',
    'CodeTether-Worker-ID',
    'CodeTether-GitHub-Installation-ID',
    'CodeTether-GitHub-App-ID',
)


def parse(message: str) -> dict[str, str]:
    """Parse unique required and optional provenance trailers."""
    labels = REQUIRED + OPTIONAL
    found: dict[str, list[str]] = {label: [] for label in labels}
    for line in message.splitlines():
        label, separator, value = line.partition(':')
        if separator and label in found and value.strip():
            found[label].append(value.strip())
    if any(len(found[label]) != 1 for label in REQUIRED):
        raise ValueError('signed provenance trailers are missing or ambiguous')
    if any(len(found[label]) > 1 for label in OPTIONAL):
        raise ValueError('signed provenance trailers are ambiguous')
    return {label: values[0] if values else '' for label, values in found.items()}
''')
    write('forgejo_provenance_keys.py', '''"""Configured verification keys for author provenance."""

import json
import os

from dataclasses import dataclass


@dataclass(frozen=True)
class ProvenanceKey:
    """One key bound to a canonical agent and tenant."""

    key_id: str
    secret: str
    agent_identity: str
    tenant_id: str
    task_auth_label: str | None


def resolve(key_id: str) -> ProvenanceKey:
    """Resolve one required key from server-only JSON configuration."""
    raw = os.environ.get('CODETETHER_PROVENANCE_SIGNING_KEYS', '')
    if not raw:
        raise RuntimeError('CodeTether provenance verification is not configured')
    try:
        values = json.loads(raw)
        value = values.get(key_id) if isinstance(values, dict) else None
    except (TypeError, ValueError) as error:
        raise RuntimeError('CodeTether provenance key configuration is invalid') from error
    if not isinstance(value, dict):
        raise ValueError('CodeTether provenance key is not trusted')
    secret = _field(value, 'secret')
    identity = _field(value, 'agent_identity')
    tenant = _field(value, 'tenant_id')
    auth_label = _field(value, 'task_auth_label')
    if not secret or not identity or not tenant:
        raise RuntimeError('CodeTether provenance key binding is incomplete')
    return ProvenanceKey(key_id, secret, identity, tenant, auth_label or None)


def _field(value: dict[object, object], field: str) -> str:
    return str(value.get(field) or '').strip()
''')
    write('forgejo_provenance_payload.py', '''"""Canonical payload shared with the Rust provenance signer."""

from collections.abc import Mapping

ORDER = (
    'CodeTether-Provenance-ID', 'CodeTether-Session-ID',
    'CodeTether-Task-ID', 'CodeTether-Run-ID', 'CodeTether-Attempt-ID',
    'CodeTether-Tenant-ID', 'CodeTether-Agent-Identity',
    'CodeTether-Agent-Name', 'CodeTether-Origin', 'CodeTether-Worker-ID',
    'CodeTether-Key-ID', 'CodeTether-GitHub-Installation-ID',
    'CodeTether-GitHub-App-ID',
)


def canonical(fields: Mapping[str, str]) -> bytes:
    """Encode the exact newline-delimited Rust signing payload."""
    return '\\n'.join(fields.get(label, '') for label in ORDER).encode()
''')
    write('forgejo_provenance_verification.py', '''"""HMAC ownership proof for signed author session provenance."""

import hashlib
import hmac

from collections.abc import Mapping

from a2a_server.forgejo_provenance_fields import parse
from a2a_server.forgejo_provenance_keys import ProvenanceKey, resolve
from a2a_server.forgejo_provenance_payload import canonical

SHA256_HEX_LENGTH = 64


def verify(message: str, metadata: Mapping[str, object]) -> ProvenanceKey:
    """Verify provenance and bind its session to the target agent and tenant."""
    fields = parse(message)
    key = resolve(fields['CodeTether-Key-ID'])
    target = str(metadata.get('target_agent_name') or '')
    if key.agent_identity != target:
        raise ValueError('provenance key is not bound to the target agent')
    if fields['CodeTether-Agent-Identity'] != target:
        raise ValueError('provenance identity does not match the target agent')
    if fields['CodeTether-Session-ID'] != metadata.get('resume_session_id'):
        raise ValueError('provenance session does not match the author session')
    if fields['CodeTether-Provenance-ID'] != metadata.get(
        'author_provenance_id'
    ):
        raise ValueError('provenance ID does not match the author envelope')
    if fields['CodeTether-Tenant-ID'] != key.tenant_id:
        raise ValueError('provenance tenant does not match the trusted key')
    signature = fields['CodeTether-Signature']
    expected = hmac.new(key.secret.encode(), canonical(fields), hashlib.sha256)
    if len(signature) != SHA256_HEX_LENGTH or not hmac.compare_digest(signature, expected.hexdigest()):
        raise ValueError('CodeTether provenance signature is invalid')
    return key
''')
    path = ROOT / 'forgejo_author_verification.py'
    text = path.read_text()
    key_import = 'from a2a_server.forgejo_provenance_keys import ProvenanceKey\n'
    if key_import not in text:
        text = text.replace(
            'from a2a_server.forgejo_commit_trailers import verify_binding\n',
            'from a2a_server.forgejo_commit_trailers import verify_binding\n' + key_import + 'from a2a_server.forgejo_provenance_verification import verify as verify_provenance\n',
        )
    text = text.replace(') -> None:\n    """Verify the current PR head, signer, and signed commit trailers."""',
        ') -> ProvenanceKey:\n    """Verify Forgejo and CodeTether proofs for the author session."""')
    anchor = "        raise ValueError('Forgejo did not verify the author commit signature')\n"
    if 'return verify_provenance(message, metadata)' not in text:
        text = text.replace(anchor, anchor + '    return verify_provenance(message, metadata)\n', 1)
    path.write_text(text)


if __name__ == '__main__':
    main()