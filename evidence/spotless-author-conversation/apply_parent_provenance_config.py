"""Declare the server-side provenance key registry secret."""

from pathlib import Path

ARGO = Path('/home/riley/A2A-Server-MCP/deploy/argocd/application.yaml')


def main() -> None:
    """Map the externally managed key registry secret into the server."""
    text = ARGO.read_text()
    anchor = '          A2A_AUTH_TOKENS: a2a-server-auth-tokens\n'
    value = (
        anchor
        + '          CODETETHER_PROVENANCE_SIGNING_KEYS: '
        + 'codetether-provenance-signing-keys\n'
    )
    if '          CODETETHER_PROVENANCE_SIGNING_KEYS:' not in text:
        if text.count(anchor) != 1:
            raise RuntimeError('Argo environment secret anchor is missing')
        ARGO.write_text(text.replace(anchor, value, 1))


if __name__ == '__main__':
    main()