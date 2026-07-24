"""Storage levels for reusable cleanup intermediates."""


def disk_only() -> object:
    """Keep large historical intermediates off executor heap."""
    from pyspark import StorageLevel

    return StorageLevel.DISK_ONLY
