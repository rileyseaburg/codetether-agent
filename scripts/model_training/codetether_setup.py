"""Install CodeTether in a Colab runtime and verify Vault credentials.

The trained adapter is evaluated through CodeTether itself, so the runtime
needs the CLI plus provider credentials read from public Vault. No secret
value is printed or persisted.
"""

import argparse
import json

from pathlib import Path

from .codetether_install import install, version
from .vault_probe import probe


def main() -> None:
    """Install the CLI, probe Vault, and record a redacted report."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--skip-install', action='store_true')
    values = parser.parse_args()
    installed = None if values.skip_install else install()
    report = {
        'install': installed,
        'version': version(),
        'vault': probe(),
    }
    values.output.write_text(
        json.dumps(report, indent=2, sort_keys=True) + '\n'
    )
    print(json.dumps(report, sort_keys=True))


if __name__ == '__main__':
    main()
