"""Character bounds and redaction predicate for governed exports."""

MIN_CHARS = 64
MAX_CHARS = 65536
"""Upper bound covering the whole governed corpus.

The earlier 8192 cap silently discarded 67,383 of 101,351 governed samples
because the median sample is 20,008 characters. Token-level bounds are
enforced later during rendering, so this stage must not drop long work.
"""


def redaction(encrypted: bool) -> str:
    """Exclude encrypted-reasoning rows only when they must be dropped."""
    if encrypted:
        return ''
    return (
        "AND json_format(CAST(messages AS JSON)) NOT LIKE '%encrypted_content%'"
    )
