"""Entry point for secured Apache Polaris catalog provisioning."""

from .catalog import ensure as ensure_catalog
from .http_client import token
from .principal import ensure as ensure_principal
from .resources import Resources
from .settings import load


def main() -> None:
    """Provision the catalog and least-privilege pipeline principal."""
    settings = load()
    access_token = token(
        settings.uri, settings.realm, settings.root_id, settings.root_secret
    )
    resources = Resources(settings.uri, settings.realm, access_token)
    ensure_catalog(resources, settings)
    ensure_principal(resources, settings)
    print('Polaris catalog and training pipeline principal are ready')


if __name__ == '__main__':
    main()
