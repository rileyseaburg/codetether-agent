"""Keep task authentication on the existing labeled A2A token registry."""

from pathlib import Path

MANIFEST = Path('/home/riley/A2A-Server-MCP/deploy/argocd/application.yaml')


def main() -> None:
    """Remove the obsolete duplicate secret mapping when present."""
    text = MANIFEST.read_text()
    entry = '          CODETETHER_TASK_AUTH_LABELS: codetether-task-auth-labels\n'
    if entry in text:
        MANIFEST.write_text(text.replace(entry, '', 1))


if __name__ == '__main__':
    main()