"""Historical MinIO connection policy tests."""

import unittest

from training_cleanup.s3_client import options


class S3ClientTests(unittest.TestCase):
    """Long-running scans tolerate slow paginated responses."""

    def test_historical_scan_uses_adaptive_long_read_retries(self) -> None:
        config = options()
        self.assertEqual(config['read_timeout'], 120)
        self.assertEqual(
            config['retries'], {'max_attempts': 10, 'mode': 'adaptive'}
        )


if __name__ == '__main__':
    unittest.main()
