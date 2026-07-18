"""Cross-runtime timestamp validation tests."""

import unittest

from training_cleanup.timestamp_validation import valid


class TimestampTests(unittest.TestCase):
    """Validate the RFC 3339 forms emitted by the Rust agent."""

    def test_nanoseconds_with_offset_are_valid(self) -> None:
        self.assertTrue(valid('2026-07-17T20:06:41.078451248+00:00'))

    def test_utc_designator_is_valid(self) -> None:
        self.assertTrue(valid('2026-07-17T20:06:41.078451248Z'))

    def test_timezone_is_required(self) -> None:
        self.assertFalse(valid('2026-07-17T20:06:41.078451248'))


if __name__ == '__main__':
    unittest.main()
