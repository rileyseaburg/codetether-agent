"""Separate Forgejo HTTP proof retrieval from proof validation."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/a2a_server")
PROOF = ROOT / "forgejo_author_proof.py"
VERIFY = ROOT / "forgejo_author_verification.py"
PROOF_CONTENT = '''"""Authenticated retrieval of Forgejo PR and commit proof."""

from collections.abc import Mapping

import httpx

from a2a_server.forgejo_verification_config import api_base
from a2a_server.forgejo_verification_response import response_json


async def fetch(
    metadata: Mapping[str, object],
    token: str,
    transport: httpx.AsyncBaseTransport | None,
) -> tuple[dict[str, object], dict[str, object]]:
    """Fetch the current PR and exact commit using a non-persisted token."""
    if not token:
        raise RuntimeError('Forgejo verification credential is required')
    host = str(metadata.get('forgejo_host') or '').lower()
    repo = str(metadata.get('repo') or '')
    number = str(metadata.get('pr_number') or '')
    head = str(metadata.get('pr_head_sha') or '').lower()
    headers = {'Authorization': f'token {token}', 'Accept': 'application/json'}
    try:
        async with httpx.AsyncClient(
            timeout=15.0, transport=transport, headers=headers
        ) as client:
            pull = await response_json(
                client, f'{api_base(host)}/repos/{repo}/pulls/{number}'
            )
            commit = await response_json(
                client, f'{api_base(host)}/repos/{repo}/git/commits/{head}'
            )
            return pull, commit
    except httpx.HTTPError as error:
        raise RuntimeError('Forgejo verification request failed') from error
'''
VERIFY_CONTENT = '''"""Independent validation of Forgejo author proof."""

from collections.abc import Mapping

import httpx

from a2a_server.forgejo_author_contract import validate
from a2a_server.forgejo_author_proof import fetch
from a2a_server.forgejo_commit_trailers import verify_binding
from a2a_server.forgejo_verification_response import nested


async def verify(
    metadata: Mapping[str, object],
    token: str,
    transport: httpx.AsyncBaseTransport | None = None,
) -> None:
    """Verify the current PR head, signer, and signed commit trailers."""
    validate(metadata)
    pull, commit = await fetch(metadata, token, transport)
    head = str(metadata.get('pr_head_sha') or '').lower()
    login = str(metadata.get('forgejo_author_login') or '').lower()
    if pull.get('state') != 'open' or nested(pull, 'head', 'sha') != head:
        raise LookupError('Forgejo PR head is no longer current')
    if str(nested(pull, 'user', 'login')).lower() != login:
        raise ValueError('Forgejo PR author does not match signed principal')
    message = nested(commit, 'commit', 'message')
    if not isinstance(message, str):
        raise ValueError('Forgejo commit message is missing')
    verify_binding(metadata, message)
    signer = nested(commit, 'commit', 'verification', 'signer', 'username')
    signer = signer or nested(commit, 'commit', 'verification', 'signer', 'login')
    verified = nested(commit, 'commit', 'verification', 'verified')
    if commit.get('sha') != head or verified is not True or str(signer).lower() != login:
        raise ValueError('Forgejo did not verify the author commit signature')
'''


def main() -> None:
    """Write the cohesive proof transport and validator modules."""
    PROOF.write_text(PROOF_CONTENT)
    VERIFY.write_text(VERIFY_CONTENT)


if __name__ == '__main__':
    main()