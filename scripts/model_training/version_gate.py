"""Fail before training when a dependency cannot be imported.

A run reached the training stage with peft unimportable and died inside
TRL's lazy module loader, which produced a confusing traceback far from the
real cause. Checking imports first turns that into one clear message.
"""

import importlib

from .version_report import PACKAGES


def main() -> None:
    """Exit non-zero listing every dependency that fails to import."""
    broken: list[str] = []
    for name in PACKAGES:
        try:
            importlib.import_module(name)
        except ImportError as error:
            broken.append(f'{name}: {error}')
    if broken:
        joined = '\n  '.join(broken)
        raise SystemExit(f'unusable dependencies:\n  {joined}')
    print('all training dependencies import cleanly')


if __name__ == '__main__':
    main()
