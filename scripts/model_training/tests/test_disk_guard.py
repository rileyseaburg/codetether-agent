"""Verify disk requirements scale with model size."""

import unittest

from model_training.disk_guard import HEADROOM_GB, LARGE_MODEL_GB, required_gb
from model_training.vram_plan import LARGE_MODEL, SMALL_MODEL


class DiskGuardTest(unittest.TestCase):
    """The 30B model needs far more disk than the 4B model."""

    def test_large_model_includes_headroom(self) -> None:
        self.assertEqual(required_gb(LARGE_MODEL), LARGE_MODEL_GB + HEADROOM_GB)

    def test_small_model_requires_less(self) -> None:
        self.assertLess(required_gb(SMALL_MODEL), required_gb(LARGE_MODEL))

    def test_unknown_model_treated_as_small(self) -> None:
        self.assertEqual(required_gb('other/model'), required_gb(SMALL_MODEL))


if __name__ == '__main__':
    unittest.main()
