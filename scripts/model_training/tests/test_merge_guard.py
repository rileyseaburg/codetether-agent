"""Verify merge memory requirements scale with model size."""

import unittest

from model_training.merge_guard import available_ram_gb, required_ram_gb
from model_training.vram_plan import LARGE_MODEL, SMALL_MODEL


class MergeGuardTest(unittest.TestCase):
    """A 30B merge needs far more host memory than a 4B merge."""

    def test_large_model_needs_more_than_sixty_gigabytes(self) -> None:
        self.assertGreater(required_ram_gb(LARGE_MODEL), 60.0)

    def test_small_model_is_cheap(self) -> None:
        self.assertLess(required_ram_gb(SMALL_MODEL), 20.0)

    def test_available_ram_is_positive_on_linux(self) -> None:
        self.assertGreater(available_ram_gb(), 0.0)


if __name__ == '__main__':
    unittest.main()
