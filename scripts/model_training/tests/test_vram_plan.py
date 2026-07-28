"""Verify model selection respects the 8 GB deployment target."""

import unittest

from model_training.constants import BASE_MODEL, MAX_LENGTH, SMALL_MODEL
from model_training.vram_plan import plan


DEPLOY_VRAM_GB = 8.0


class VramPlanTest(unittest.TestCase):
    """The served card, not the training card, bounds model size."""

    def test_capable_device_selects_the_nine_billion_model(self) -> None:
        result = plan(80.0)
        self.assertEqual(result['recommended_model'], BASE_MODEL)
        self.assertEqual(result['max_length'], MAX_LENGTH)

    def test_mid_range_device_selects_the_four_billion_model(self) -> None:
        self.assertEqual(plan(16.0)['recommended_model'], SMALL_MODEL)

    def test_tiny_device_shortens_sequences(self) -> None:
        self.assertLess(int(plan(10.0)['max_length']), MAX_LENGTH)

    def test_every_choice_serves_on_the_target_card(self) -> None:
        """A model that cannot be served is never worth training."""
        for gigabytes in (80.0, 40.0, 16.0, 10.0):
            served = float(plan(gigabytes)['serves_on_8gb'])
            self.assertLessEqual(served, DEPLOY_VRAM_GB)


if __name__ == '__main__':
    unittest.main()
