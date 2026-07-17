"""Idempotent Polaris management-resource operations."""

from .http_client import call


class Resources:
    """Create catalog security resources when absent."""

    def __init__(self, uri: str, realm: str, token: str) -> None:
        self.uri = uri
        self.realm = realm
        self.token = token

    def ensure(self, get_path: str, post_path: str, payload: object) -> None:
        """POST a resource only when its GET endpoint returns 404."""
        status, _ = self._call('GET', get_path)
        if status == 404:
            status, _ = self._call('POST', post_path, payload)
            if status not in {200, 201, 204, 409}:
                raise RuntimeError(
                    f'Polaris create failed with status {status}'
                )

    def put(self, path: str, payload: object) -> None:
        """Apply an idempotent grant or role assignment."""
        status, _ = self._call('PUT', path, payload)
        if status not in {200, 201, 204, 409}:
            raise RuntimeError(f'Polaris update failed with status {status}')

    def get(self, path: str) -> object | None:
        """Read a management resource that must exist."""
        status, body = self._call('GET', path)
        if status != 200:
            raise RuntimeError(f'Polaris read failed with status {status}')
        return body

    def post(self, path: str, payload: object) -> object | None:
        """Apply an update whose API uses POST."""
        status, body = self._call('POST', path, payload)
        if status not in {200, 201, 204}:
            raise RuntimeError(f'Polaris update failed with status {status}')
        return body

    def _call(
        self, method: str, path: str, payload: object | None = None
    ) -> tuple[int, object | None]:
        return call(method, self.uri, self.realm, self.token, path, payload)
