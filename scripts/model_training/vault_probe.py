"""Report which Vault provider credentials are reachable.

Only presence is reported. Secret values are never returned, printed, or
written to disk.
"""

import os

from .vault_client import read


PROVIDERS = ('openai', 'anthropic', 'zhipuai', 'huggingface', 'openrouter')


def probe() -> dict[str, object]:
    """Return reachable providers without exposing any secret value."""
    address = os.environ.get('VAULT_ADDR')
    if not address or not os.environ.get('VAULT_TOKEN'):
        return {'configured': False, 'available': []}
    available: list[str] = []
    missing: list[str] = []
    for provider in PROVIDERS:
        try:
            fields = read(provider)
        except (KeyError, OSError):
            missing.append(provider)
            continue
        available.append(provider)
        del fields
    return {
        'configured': True,
        'address': address,
        'available': available,
        'missing': missing,
    }
