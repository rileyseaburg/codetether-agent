"""Polaris catalog and catalog-role provisioning."""

from .grants import ensure_catalog_grant
from .resources import Resources
from .settings import Settings


CATALOG = 'codetether'
CATALOG_ROLE = 'training-writer'


def ensure(resources: Resources, settings: Settings) -> None:
    """Create the MinIO-backed catalog and its writer role."""
    catalog = {
        'catalog': {
            'name': CATALOG,
            'type': 'INTERNAL',
            'readOnly': False,
            'properties': {'default-base-location': settings.storage_location},
            'storageConfigInfo': {
                'storageType': 'S3',
                'allowedLocations': [settings.storage_location],
                'endpoint': settings.storage_endpoint,
                'endpointInternal': settings.storage_endpoint,
                'pathStyleAccess': True,
            },
        }
    }
    resources.ensure(f'/catalogs/{CATALOG}', '/catalogs', catalog)
    role = {'catalogRole': {'name': CATALOG_ROLE}}
    base = f'/catalogs/{CATALOG}/catalog-roles'
    resources.ensure(f'{base}/{CATALOG_ROLE}', base, role)
    ensure_catalog_grant(resources, f'{base}/{CATALOG_ROLE}/grants')
