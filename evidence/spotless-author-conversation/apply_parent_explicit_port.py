"""Align allowlisted explicit-port hosts across identity runtimes."""

from pathlib import Path

CONFIG = Path('/home/riley/A2A-Server-MCP/a2a_server/forgejo_verification_config.py')
TEST = Path('/home/riley/A2A-Server-MCP/tests/test_forgejo_verification_config.py')


def main() -> None:
    """Compare the configured authority including its explicit port."""
    text = CONFIG.read_text().replace(
        "        or parsed.hostname != host.lower()\n",
        "        or parsed.netloc.lower() != host.lower()\n",
    )
    text = text.replace(
        "    return {parsed.hostname: single} if parsed.hostname else {}\n",
        "    return {parsed.netloc.lower(): single} if parsed.hostname else {}\n",
    )
    CONFIG.write_text(text)
    test = TEST.read_text()
    addition = '''

def test_configured_https_host_with_explicit_port_is_accepted(monkeypatch):
    value = {'forge.example:8443': 'https://forge.example:8443/api/v1'}
    monkeypatch.setenv('CODETETHER_FORGEJO_API_BASE_URLS', json.dumps(value))
    assert api_base('forge.example:8443') == value['forge.example:8443']
'''
    if 'test_configured_https_host_with_explicit_port_is_accepted' not in test:
        TEST.write_text(test.rstrip() + addition)


if __name__ == '__main__':
    main()