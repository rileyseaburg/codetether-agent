"""Stub trainer that stays alive so launch verification is meaningful."""

import sys
import time


LIFETIME_SECONDS = 60


def main() -> None:
    """Log a start line, then idle long enough to be observed."""
    print(f'stub trainer started with {sys.argv[1:]}', flush=True)
    time.sleep(LIFETIME_SECONDS)


if __name__ == '__main__':
    main()
