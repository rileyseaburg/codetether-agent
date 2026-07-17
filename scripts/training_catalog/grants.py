"""Idempotent catalog privilege grants."""

from .resources import Resources


PRIVILEGE = 'CATALOG_MANAGE_CONTENT'


def ensure_catalog_grant(resources: Resources, path: str) -> None:
    """Grant catalog content management only when it is absent."""
    body = resources.get(path)
    grants = body.get('grants', []) if isinstance(body, dict) else []
    present = any(
        isinstance(grant, dict)
        and grant.get('type') == 'catalog'
        and grant.get('privilege') == PRIVILEGE
        for grant in grants
    )
    if not present:
        resources.put(
            path, {'grant': {'type': 'catalog', 'privilege': PRIVILEGE}}
        )
