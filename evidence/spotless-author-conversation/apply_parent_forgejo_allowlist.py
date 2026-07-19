"""Configure the production Forgejo verification endpoint explicitly."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/deploy/argocd/application.yaml")
ANCHOR = '''          A2A_AGENT_URL: https://api.codetether.run
'''
ADDITION = '''          CODETETHER_FORGEJO_API_BASE_URLS: '{"forgejo.quantum-forge.io":"https://forgejo.quantum-forge.io/api/v1"}'
'''


def main() -> None:
    """Add the host map without embedding a credential."""
    text = PATH.read_text()
    if ADDITION.strip() in text:
        return
    if text.count(ANCHOR) != 1:
        raise RuntimeError("production Forgejo allowlist anchor is missing")
    PATH.write_text(text.replace(ANCHOR, ANCHOR + ADDITION, 1))


if __name__ == "__main__":
    main()