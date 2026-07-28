"""Verify the masking gate tolerates expected truncation losses.

Blocking on any fully masked example failed a healthy run where 5 of 200
sampled pairs, 2.5 percent, exceeded the sequence window.
"""

import unittest

from model_training.mask_gate import (
    MAX_FULLY_MASKED_RATE,
    MIN_SUPERVISED_FRACTION,
    gate,
)


def _report(masked: int, supervised: float) -> dict[str, object]:
    return {
        'pairs_sampled': 200,
        'fully_masked': masked,
        'supervised_fraction': supervised,
    }


class MaskGateTest(unittest.TestCase):
    """A small truncation rate is normal; a systemic one is not."""

    def test_small_masked_rate_passes(self) -> None:
        gate(_report(5, 0.028))

    def test_systemic_masking_fails(self) -> None:
        over = int(200 * MAX_FULLY_MASKED_RATE) + 1
        with self.assertRaises(SystemExit):
            gate(_report(over, 0.028))

    def test_absent_supervision_fails(self) -> None:
        with self.assertRaises(SystemExit):
            gate(_report(0, MIN_SUPERVISED_FRACTION / 2))


if __name__ == '__main__':
    unittest.main()
