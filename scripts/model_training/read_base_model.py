"""Print the pinned base model identifier.

Shell scripts previously carried their own literal default, which drifted
from the pinned constant and requested the wrong machine tier.
"""

from .constants import BASE_MODEL


def main() -> None:
    """Write the base model identifier to stdout."""
    print(BASE_MODEL, end='')


if __name__ == '__main__':
    main()
