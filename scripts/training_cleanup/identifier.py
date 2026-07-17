"""Validation for SQL identifiers supplied to the Spark job."""

import re


_IDENTIFIER = re.compile(r'^[A-Za-z_][A-Za-z0-9_]*$')


def checked(value: str, label: str) -> str:
    """Return a safe SQL identifier or terminate argument loading."""
    if not _IDENTIFIER.fullmatch(value):
        raise SystemExit(f'{label} must be an unquoted SQL identifier')
    return value
