"""Environment-backed command setting construction."""

import os

from datetime import datetime, timezone

from .identifier import checked
from .settings import Settings
from .settings_args import parser


def load() -> Settings:
    """Parse and validate Spark cleanup settings."""
    args = parser().parse_args()
    endpoint = args.endpoint or os.getenv('MINIO_ENDPOINT')
    if not endpoint:
        raise SystemExit('MINIO_ENDPOINT or --endpoint is required')
    current_hour = datetime.now(timezone.utc).strftime('%Y/%m/%d/%H')
    source_before = args.source_before or (
        f'{args.source_prefix.strip("/")}/{current_hour}/'
    )
    return Settings(
        endpoint=endpoint,
        bucket=args.bucket,
        source_prefix=args.source_prefix,
        source_before=source_before,
        catalog=checked(args.catalog, 'catalog'),
        namespace=checked(args.namespace, 'namespace'),
        table_prefix=checked(args.table_prefix, 'table prefix'),
        run_id=args.run_id
        or datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ'),
        max_content_chars=args.max_content_chars,
        min_partitions=args.min_partitions,
        apply=args.apply,
    )
