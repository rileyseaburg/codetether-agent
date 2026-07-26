"""Verify VRAM-based model selection reflects measured memory cost."""

import unittest

from model_training.memory_budget import estimate, fits
from model_training.vram_plan import LARGE_MODEL, plan


class VramPlanTest(unittest.TestCase):
    """Device capacity must decide the trainable model size."""

    def test_eighty_gigabytes_trains_thirty_billion(self) -> None:
        result = plan(80.0)
        self.assertEqual(result['recommended_model'], LARGE_MODEL)
        self.assertTrue(result['mixture_of_experts'])

    def test_forty_gigabytes_avoids_thirty_billion(self) -> None:
        """An A100 40 GB hit CUDA OOM at 39.47 GiB with the 30B model."""
        result = plan(42.4)
        self.assertIn('4B', str(result['recommended_model']))

    def test_moe_overhead_exceeds_flat_estimate(self) -> None:
        flat = estimate(30.5, 4096, False)['total_gb']
        moe = estimate(30.5, 4096, True)['total_gb']
        self.assertGreater(moe, flat)
        self.assertGreater(moe, 39.47)

    def test_thirty_billion_does_not_fit_forty_gigabytes(self) -> None:
        self.assertFalse(fits(30.5, 4096, 40.0, True))
        self.assertTrue(fits(30.5, 4096, 80.0, True))

    def test_plan_estimate_fits_reported_device(self) -> None:
        result = plan(80.0)
        estimated = result['estimated']
        self.assertIsInstance(estimated, dict)
        self.assertLess(float(estimated['total_gb']), 80.0)


if __name__ == '__main__':
    unittest.main()
