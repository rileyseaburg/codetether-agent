"""Verify merge memory requirements scale with model size."""

import unittest

from model_training.constants import BASE_MODEL, SMALL_MODEL
from model_training.merge_guard import available_ram_gb, required_ram_gb


class MergeGuardTest(unittest.TestCase):
    """Merging dequantizes on the CPU, so host memory is the limit."""

    def test_base_model_needs_more_than_twenty_gigabytes(self) -> None:
        self.assertGreater(required_ram_gb(BASE_MODEL), 20.0)

    def test_small_model_is_cheaper(self) -> None:
        self.assertLess(
            required_ram_gb(SMALL_MODEL), required_ram_gb(BASE_MODEL)
        )

    def test_available_ram_is_positive_on_linux(self) -> None:
        self.assertGreater(available_ram_gb(), 0.0)


if __name__ == '__main__':
    unittest.main()
