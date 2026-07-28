"""Verify checkpoint cadence bounds preemption loss.

Training runs on SPOT capacity, which is reclaimed without warning, so the
save interval determines how much progress a preemption can destroy.
"""

import unittest

from model_training.checkpoint_policy import CHECKPOINTING


SECONDS_PER_STEP = 17.26
MAX_ACCEPTABLE_LOSS_MINUTES = 45


class CheckpointPolicyTest(unittest.TestCase):
    """Saves must coincide with evaluations and stay frequent."""

    def test_saves_align_with_evaluations(self) -> None:
        """load_best_model_at_end requires a checkpoint per evaluation."""
        self.assertEqual(
            CHECKPOINTING['save_steps'], CHECKPOINTING['eval_steps']
        )

    def test_preemption_loss_is_bounded(self) -> None:
        minutes = CHECKPOINTING['save_steps'] * SECONDS_PER_STEP / 60
        self.assertLessEqual(minutes, MAX_ACCEPTABLE_LOSS_MINUTES)

    def test_best_checkpoint_is_selected_by_eval_loss(self) -> None:
        self.assertTrue(CHECKPOINTING['load_best_model_at_end'])
        self.assertEqual(CHECKPOINTING['metric_for_best_model'], 'eval_loss')
        self.assertFalse(CHECKPOINTING['greater_is_better'])


if __name__ == '__main__':
    unittest.main()
