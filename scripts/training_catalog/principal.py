"""Least-privilege Polaris pipeline principal provisioning."""

from .assignments import ensure_named
from .catalog import CATALOG, CATALOG_ROLE
from .resources import Resources
from .settings import Settings


PRINCIPAL = 'training-pipeline'
PRINCIPAL_ROLE = 'training-pipeline-role'


def ensure(resources: Resources, settings: Settings) -> None:
    """Create stable pipeline credentials and attach its writer role."""
    resources.ensure(
        f'/principals/{PRINCIPAL}',
        '/principals',
        {'principal': {'name': PRINCIPAL}, 'credentialRotationRequired': False},
    )
    resources.post(
        f'/principals/{PRINCIPAL}/reset',
        {
            'clientId': settings.pipeline_id,
            'clientSecret': settings.pipeline_secret,
        },
    )
    role = {'principalRole': {'name': PRINCIPAL_ROLE}}
    resources.ensure(
        f'/principal-roles/{PRINCIPAL_ROLE}', '/principal-roles', role
    )
    ensure_named(
        resources,
        f'/principals/{PRINCIPAL}/principal-roles',
        PRINCIPAL_ROLE,
        role,
    )
    ensure_named(
        resources,
        f'/principal-roles/{PRINCIPAL_ROLE}/catalog-roles/{CATALOG}',
        CATALOG_ROLE,
        {'catalogRole': {'name': CATALOG_ROLE}},
    )
