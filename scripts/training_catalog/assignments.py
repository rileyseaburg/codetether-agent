"""Idempotent Polaris role assignments."""

from .resources import Resources


def ensure_named(
    resources: Resources, path: str, name: str, payload: object
) -> None:
    """Assign a named role only when the list endpoint does not contain it."""
    body = resources.get(path)
    roles = body.get('roles', []) if isinstance(body, dict) else []
    present = any(
        isinstance(role, dict) and role.get('name') == name for role in roles
    )
    if not present:
        resources.put(path, payload)
