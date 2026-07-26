"""Resolve a HuggingFace token from Vault, then the environment.

Vault is the primary source so credentials never live in notebooks or the
repository. Environment variables remain a local-development fallback.
"""

import os

from .vault_client import token_for


FIELDS = ('token', 'api_key', 'hf_token', 'access_token')
PROVIDERS = ('huggingface', 'hf')
ENV_VARS = ('HF_TOKEN', 'HUGGINGFACE_API_KEY', 'HUGGING_FACE_HUB_TOKEN')


def resolve_token() -> str:
    """Return a HuggingFace token or raise with actionable guidance."""
    if os.environ.get('VAULT_ADDR') and os.environ.get('VAULT_TOKEN'):
        for provider in PROVIDERS:
            try:
                return token_for(provider, *FIELDS)
            except (KeyError, OSError):
                continue
    for name in ENV_VARS:
        value = os.environ.get(name)
        if value and value.strip():
            return value.strip()
    raise SystemExit(
        'No HuggingFace token found. Store one in Vault at '
        'secret/codetether/providers/huggingface with a "token" field, '
        'or set HF_TOKEN for local runs.'
    )
