"""Verify the promotion gate rejects the historical v2 regression."""

import unittest

from model_training.gate_rules import decide


BASE = {'code_pass_rate': 0.75, 'tool_call_rate': 0.50}
BASE_EMPTY = {'empty_rate': 0.0}


class GateRulesTest(unittest.TestCase):
    """Loss improvements must never substitute for behaviour."""

    def test_v2_regression_is_rejected(self) -> None:
        """v2 scored 0.00 code pass and 0.50 empty rate despite lower loss."""
        candidate = {'code_pass_rate': 0.00, 'tool_call_rate': 0.00}
        verdict = decide(BASE, candidate, BASE_EMPTY, {'empty_rate': 0.50})
        self.assertFalse(verdict['promote'])
        failed = {c['name'] for c in verdict['checks'] if not c['passed']}
        self.assertEqual(
            failed,
            {
                'code_pass_rate_not_worse',
                'empty_rate_within_budget',
                'tool_calls_emitted',
            },
        )

    def test_improved_candidate_is_promoted(self) -> None:
        candidate = {'code_pass_rate': 0.80, 'tool_call_rate': 0.60}
        verdict = decide(BASE, candidate, BASE_EMPTY, {'empty_rate': 0.0})
        self.assertTrue(verdict['promote'])

    def test_silent_model_is_rejected(self) -> None:
        """Matching code quality cannot excuse never calling tools."""
        candidate = {'code_pass_rate': 0.75, 'tool_call_rate': 0.00}
        verdict = decide(BASE, candidate, BASE_EMPTY, {'empty_rate': 0.0})
        self.assertFalse(verdict['promote'])


if __name__ == '__main__':
    unittest.main()
