"""Minimal authenticated HTTP client for Apache Polaris."""

import json

from base64 import b64encode
from urllib.error import HTTPError
from urllib.parse import urlencode
from urllib.request import Request, urlopen


def token(uri: str, realm: str, client_id: str, secret: str) -> str:
    """Exchange client credentials for a Polaris access token."""
    credentials = b64encode(f'{client_id}:{secret}'.encode()).decode()
    data = urlencode(
        {'grant_type': 'client_credentials', 'scope': 'PRINCIPAL_ROLE:ALL'}
    ).encode()
    request = Request(f'{uri}/api/catalog/v1/oauth/tokens', data=data)
    request.add_header('Authorization', f'Basic {credentials}')
    request.add_header('Polaris-Realm', realm)
    with urlopen(request, timeout=30) as response:
        return json.load(response)['access_token']


def call(
    method: str,
    uri: str,
    realm: str,
    access_token: str,
    path: str,
    payload: object | None = None,
) -> tuple[int, object | None]:
    """Call a management endpoint and return status plus decoded JSON."""
    data = json.dumps(payload).encode() if payload is not None else None
    request = Request(
        f'{uri}/api/management/v1{path}', data=data, method=method
    )
    request.add_header('Authorization', f'Bearer {access_token}')
    request.add_header('Polaris-Realm', realm)
    request.add_header('Content-Type', 'application/json')
    try:
        with urlopen(request, timeout=30) as response:
            body = response.read()
            return response.status, json.loads(body) if body else None
    except HTTPError as error:
        if error.code in {404, 409}:
            return error.code, None
        raise
