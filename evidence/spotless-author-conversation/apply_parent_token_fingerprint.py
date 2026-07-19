"""Make legacy-token replay scopes unguessable without persisting secrets."""

from pathlib import Path

SCOPE = Path("/home/riley/A2A-Server-MCP/a2a_server/task_auth_scope.py")
TEST = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_request_scope.py")


def main() -> None:
    """Add a one-way bearer fingerprint to the internal scope."""
    text = SCOPE.read_text()
    if 'import hashlib\n' not in text:
        text = text.replace('import hmac\n', 'import hashlib\nimport hmac\n', 1)
    old = "            return f'token:{label}'\n"
    new = '''            fingerprint = hashlib.sha256(supplied.encode()).hexdigest()[:32]
            return f'token:{label}:{fingerprint}'
'''
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('token scope return anchor is missing')
        text = text.replace(old, new, 1)
    SCOPE.write_text(text)
    test = TEST.read_text()
    old_assert = "    assert resolve(request('secret')) == ('token:reviewer', None)\n"
    new_assert = '''    scope, tenant = resolve(request('secret'))
    assert scope.startswith('token:reviewer:')
    assert 'secret' not in scope
    assert tenant is None
'''
    if new_assert not in test:
        if test.count(old_assert) != 1:
            raise RuntimeError('token scope assertion anchor is missing')
        TEST.write_text(test.replace(old_assert, new_assert, 1))


if __name__ == '__main__':
    main()