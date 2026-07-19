"""Install server-derived conversation continuity validation externally."""

from pathlib import Path

MODULE = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_conversation_identity.py")
IDENTITY = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_identity.py")
TEST = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_protocol.py")
CONTENT = '''"""Deterministic Forgejo PR conversation identity validation."""

import hashlib
import re
from typing import Any, Dict

_REPOSITORY = re.compile(r'^[a-z0-9._-]{1,128}/[a-z0-9._-]{1,128}$')


def conversation_id(repo: str, pr_number: Any, target: str) -> str:
    """Derive one reusable conversation for a repository PR and author."""
    repo = repo.lower()
    if not _REPOSITORY.fullmatch(repo):
        raise ValueError('invalid Forgejo repository identity')
    try:
        number = int(pr_number)
    except (TypeError, ValueError) as error:
        raise ValueError('invalid Forgejo PR number') from error
    if number <= 0 or str(number) != str(pr_number):
        raise ValueError('invalid Forgejo PR number')
    digest = hashlib.sha256(f'{repo}\\n{number}\\n{target}'.encode()).hexdigest()
    return f'forgejo_pr_{digest[:48]}'


def validate_binding(metadata: Dict[str, Any], target: str) -> None:
    """Reject aliases that disagree with the server-derived binding."""
    expected = conversation_id(metadata.get('repo', ''), metadata.get('pr_number'), target)
    if metadata.get('context_id') != expected or metadata.get('conversation_id') != expected:
        raise ValueError('conversation does not match the Forgejo PR author')
    if metadata.get('author_agent_identity') != target:
        raise ValueError('author identity alias does not match the target')
    if metadata.get('head_sha') != metadata.get('pr_head_sha'):
        raise ValueError('head SHA aliases do not match')
    login = str(metadata.get('forgejo_author_login') or '').lower()
    if str(metadata.get('git_signer') or '').lower() != f'forgejo:{login}':
        raise ValueError('verified signer does not match the Forgejo author')
'''


def insert_once(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"conversation contract anchor missing: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Write the validator and connect it to source and fixtures."""
    MODULE.write_text(CONTENT)
    insert_once(
        IDENTITY,
        "from typing import Any, Dict\n",
        "\nfrom .forgejo_conversation_identity import validate_binding\n",
    )
    insert_once(
        IDENTITY,
        "    if metadata.get('provenance_verified') is not True or metadata.get('preserve_session_workspace') is not True:\n        raise ValueError('verified workspace preservation is required')\n",
        "    validate_binding(metadata, target)\n",
    )
    insert_once(
        TEST,
        "from a2a_server.forgejo_author_identity import canonical_identity, validate\n",
        "from a2a_server.forgejo_conversation_identity import conversation_id\n",
    )
    insert_once(
        TEST,
        "        'preserve_session_workspace': True,\n",
        "        'author_agent_identity': target,\n        'head_sha': 'a' * 40,\n        'git_signer': 'forgejo:alice',\n        'context_id': conversation_id('owner/repo', 42, target),\n        'conversation_id': conversation_id('owner/repo', 42, target),\n",
    )


if __name__ == "__main__":
    main()