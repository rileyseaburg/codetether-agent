"""Sharded S3 listing policy tests."""

import unittest

from unittest.mock import MagicMock, patch

from training_cleanup.s3_listing import keys
from training_cleanup.settings import Settings


class ListingTests(unittest.TestCase):
    """Validate cutoff, extension, ordering, and duplicate handling."""

    def test_only_historical_jsonl_keys_are_returned(self) -> None:
        settings = Settings(
            'http://minio',
            'bucket',
            'training/v2/',
            'training/v2/2026/02/',
            'iceberg',
            'codetether',
            'training',
            'run',
            100,
            65_536,
            96,
            4,
            False,
            False,
        )
        paginator = MagicMock()
        paginator.paginate.return_value = [
            {
                'Contents': [
                    {'Key': 'training/v2/2026/01/a.jsonl'},
                    {'Key': 'training/v2/2026/01/a.jsonl'},
                    {'Key': 'training/v2/2026/01/notes.txt'},
                    {'Key': 'training/v2/2026/02/a.jsonl'},
                ]
            }
        ]
        client = MagicMock()
        client.get_paginator.return_value = paginator
        with (
            patch('training_cleanup.s3_listing.create', return_value=client),
            patch(
                'training_cleanup.s3_listing.hour_shards',
                return_value=['shard/'],
            ),
        ):
            self.assertEqual(keys(settings), ['training/v2/2026/01/a.jsonl'])


if __name__ == '__main__':
    unittest.main()
