"""Container command that bootstraps itself inside a Vertex worker.

A prebuilt training image contains no project code, so referencing a path
such as `/workspace/ct/...` fails with exit status 127, command not found,
before anything can clone the repository. The command therefore clones the
repository first and only then executes the entrypoint.
"""

REPO = 'https://github.com/rileyseaburg/codetether-training'
BUNDLE = '/workspace/ct'
ENTRYPOINT = 'scripts/model_training/vertex_entrypoint.sh'


def container_command(repo: str = REPO, bundle: str = BUNDLE) -> list[str]:
    """Return a shell command that clones the repository, then trains."""
    script = (
        'set -euo pipefail; '
        f'git clone --depth 1 {repo} {bundle}; '
        f'exec bash {bundle}/{ENTRYPOINT}'
    )
    return ['/bin/bash', '-c', script]
