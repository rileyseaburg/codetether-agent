"""Report installed library versions for run reproducibility.

Written as a module rather than an inline `python3 -c` string: multi-line
shell continuations inside quotes produced invalid Python and aborted the
container under `set -e` before any diagnostic reached Cloud Storage.
"""

import importlib
import json


PACKAGES = (
    'torch',
    'transformers',
    'trl',
    'peft',
    'accelerate',
    'bitsandbytes',
)


def main() -> None:
    """Print the resolved version of each training dependency."""
    report: dict[str, str] = {}
    for name in PACKAGES:
        try:
            module = importlib.import_module(name)
        except ImportError as error:
            report[name] = f'missing: {error}'
            continue
        report[name] = str(getattr(module, '__version__', 'unknown'))
    print(json.dumps(report, sort_keys=True))


if __name__ == '__main__':
    main()
