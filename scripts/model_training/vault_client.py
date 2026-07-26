"""Read provider credentials from the public Vault KV store.

Secrets are never written to the repository or to generated notebooks. The
Colab bootstrap reads them at runtime from Vault using a token supplied in
the environment.
"""

import json
import os
import urllib.request


DEFAULT_MOUNT = 'secret'
DEFAULT_PREFIX = 'codetether/providers'
USER_AGENT = 'codetether-training/1.0'
"""The public Vault endpoint sits behind a WAF that rejects the default
`Python-urllib` user agent with HTTP 403, so an explicit agent is required.
"""


def read(provider: str) -> dict[str, object]:
    """Return the KV data for one provider."""
    address = os.environ['VAULT_ADDR'].rstrip('/')
    token = os.environ['VAULT_TOKEN']
    mount = os.environ.get('VAULT_MOUNT', DEFAULT_MOUNT)
    prefix = os.environ.get('VAULT_SECRETS_PATH', DEFAULT_PREFIX)
    url = f'{address}/v1/{mount}/data/{prefix}/{provider}'
    headers = {'X-Vault-Token': token, 'User-Agent': USER_AGENT}
    request = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(request, timeout=20) as response:
        body = json.load(response)
    data = body.get('data', {}).get('data')
    if not isinstance(data, dict):
        raise KeyError(f'no secret data at {mount}/{prefix}/{provider}')
    return data


def token_for(provider: str, *fields: str) -> str:
    """Return the first populated credential field for a provider."""
    data = read(provider)
    for field in fields:
        value = data.get(field)
        if isinstance(value, str) and value.strip():
            return value
    raise KeyError(f'no credential field {fields} for {provider}')
