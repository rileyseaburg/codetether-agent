"""Split canonical identity derivation from task contract validation."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
IDENTITY = ROOT / "a2a_server/forgejo_author_identity.py"
CONTRACT = ROOT / "a2a_server/forgejo_author_contract.py"
TASK = ROOT / "a2a_server/forgejo_author_task.py"
VERIFY = ROOT / "a2a_server/forgejo_author_verification.py"
TESTS = [
    ROOT / "tests/test_forgejo_author_protocol.py",
    ROOT / "tests/test_forgejo_conversation_identity.py",
]
IDENTITY_CONTENT = '''"""Canonical identities for configured Forgejo agent principals."""

import hashlib
import re

_COMPONENT = re.compile(r'^[a-z0-9][a-z0-9._-]{0,127}$')
_HOST = re.compile(r'^[a-z0-9][a-z0-9.:-]{0,127}$')


def canonical_identity(host: str, login: str, slot: str) -> str:
    """Derive a stable route from one verified Forgejo principal."""
    host, login, slot = host.lower(), login.lower(), slot.lower()
    if not _HOST.fullmatch(host) or not all(
        _COMPONENT.fullmatch(value) for value in (login, slot)
    ):
        raise ValueError('unsafe Forgejo principal')
    digest = hashlib.sha256(f'{host}\\n{login}\\n{slot}'.encode()).hexdigest()
    return f'ctforgejo_{digest[:40]}'
'''
CONTRACT_CONTENT = '''"""Fail-closed metadata contract for Forgejo author tasks."""

import re
from typing import Mapping

from .forgejo_author_identity import canonical_identity
from .forgejo_conversation_identity import validate_binding

PROTOCOL = 'codetether.forgejo-author.v1'
_HEAD = re.compile(r'^[0-9a-f]{40}$')
_SESSION = re.compile(r'^[A-Za-z0-9_-]{1,128}$')
_PROVENANCE = re.compile(r'^ctprov_[A-Za-z0-9_-]{16,80}$')


def validate(metadata: Mapping[str, object]) -> str:
    """Validate and return the canonical target identity."""
    if metadata.get('protocol') != PROTOCOL:
        raise ValueError('unsupported author-conversation protocol')
    if metadata.get('source') != 'forgejo-pr-review':
        raise ValueError('invalid author-conversation source')
    if metadata.get('workflow_stage') != 'forgejo-author-review':
        raise ValueError('invalid author-conversation stage')
    host = str(metadata.get('forgejo_host') or '').lower()
    login = str(metadata.get('forgejo_author_login') or '').lower()
    slot = str(metadata.get('agent_slot') or '').lower()
    target = canonical_identity(host, login, slot)
    if metadata.get('target_agent_name') != target:
        raise ValueError('target does not match the signed Forgejo principal')
    if not _HEAD.fullmatch(str(metadata.get('pr_head_sha') or '').lower()):
        raise ValueError('invalid immutable head SHA')
    if not _SESSION.fullmatch(str(metadata.get('resume_session_id') or '')):
        raise ValueError('invalid reusable session identity')
    if not _PROVENANCE.fullmatch(str(metadata.get('author_provenance_id') or '')):
        raise ValueError('invalid signed provenance identity')
    if metadata.get('provenance_verified') is not True:
        raise ValueError('verified provenance is required')
    if metadata.get('preserve_session_workspace') is not True:
        raise ValueError('verified workspace preservation is required')
    validate_binding(metadata, target)
    return target
'''


def main() -> None:
    """Write focused modules and update all consumers."""
    IDENTITY.write_text(IDENTITY_CONTENT)
    CONTRACT.write_text(CONTRACT_CONTENT)
    for path in [TASK, VERIFY]:
        text = path.read_text().replace(
            'from .forgejo_author_identity import validate',
            'from .forgejo_author_contract import validate',
        )
        path.write_text(text)
    for path in TESTS:
        text = path.read_text().replace(
            'from a2a_server.forgejo_author_identity import canonical_identity, validate',
            'from a2a_server.forgejo_author_contract import validate\nfrom a2a_server.forgejo_author_identity import canonical_identity',
        ).replace(
            'from a2a_server.forgejo_author_identity import validate',
            'from a2a_server.forgejo_author_contract import validate',
        )
        path.write_text(text)


if __name__ == "__main__":
    main()