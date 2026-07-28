"""Verify checkpoint ranking rejects the v2 failure profile."""

import unittest

from model_training.checkpoint_rank import rank


class CheckpointRankTest(unittest.TestCase):
    """Lowest loss must never outrank usable behaviour."""

    def test_silent_checkpoint_is_excluded(self) -> None:
        """The v2 model answered nothing for half of all short prompts."""
        records = [
            {'step': 500, 'empty_rate': 0.5, 'tool_call_rate': 0.9},
            {'step': 1000, 'empty_rate': 0.0, 'tool_call_rate': 0.4},
        ]
        verdict = rank(records)
        self.assertEqual(verdict['best_step'], 1000)
        self.assertEqual(verdict['within_empty_budget'], 1)
        self.assertFalse(verdict['degraded'])

    def test_highest_tool_rate_wins_among_usable(self) -> None:
        records = [
            {'step': 100, 'empty_rate': 0.0, 'tool_call_rate': 0.2},
            {'step': 200, 'empty_rate': 0.0, 'tool_call_rate': 0.7},
        ]
        self.assertEqual(rank(records)['best_step'], 200)

    def test_all_silent_reports_degraded(self) -> None:
        records = [
            {'step': 100, 'empty_rate': 0.6, 'tool_call_rate': 0.1},
            {'step': 200, 'empty_rate': 0.9, 'tool_call_rate': 0.3},
        ]
        verdict = rank(records)
        self.assertTrue(verdict['degraded'])
        self.assertEqual(verdict['within_empty_budget'], 0)


if __name__ == '__main__':
    unittest.main()
