"""Training length must match the length the corpus was rendered at.

A run configured for 4,096 tokens against pairs rendered at 8,192 silently
discarded 29.5 percent of training examples: every pair whose prompt alone
filled the window lost all completion tokens to truncation.
"""

import unittest

from model_training.constants import MAX_LENGTH
from model_training.vram_plan import plan


class SequenceAgreementTest(unittest.TestCase):
    """Capable devices must train at the rendered sequence length."""

    def test_large_device_matches_render_length(self) -> None:
        self.assertEqual(plan(85.1)['max_length'], MAX_LENGTH)

    def test_medium_device_matches_render_length(self) -> None:
        self.assertEqual(plan(42.4)['max_length'], MAX_LENGTH)

    def test_tiny_device_may_shorten_with_known_cost(self) -> None:
        """Below the 4B training threshold sequences must shorten."""
        self.assertLess(int(plan(10.0)['max_length']), MAX_LENGTH)


if __name__ == '__main__':
    unittest.main()
