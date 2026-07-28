"""Verify disk requirements scale with model size."""

import unittest

from model_training.constants import BASE_MODEL, SMALL_MODEL
from model_training.disk_guard import BASE_MODEL_GB, HEADROOM_GB, required_gb


class DiskGuardTest(unittest.TestCase):
    """Weights, checkpoints, and the merged export all share the disk."""

    def test_base_model_includes_headroom(self) -> None:
        self.assertEqual(required_gb(BASE_MODEL), BASE_MODEL_GB + HEADROOM_GB)

    def test_small_model_requires_less(self) -> None:
        self.assertLess(required_gb(SMALL_MODEL), required_gb(BASE_MODEL))

    def test_unknown_model_treated_as_small(self) -> None:
        self.assertEqual(required_gb('other/model'), required_gb(SMALL_MODEL))


if __name__ == '__main__':
    unittest.main()
