"""Verify adapters avoid per-expert projections on MoE models."""

import unittest

from model_training.constants import TARGET_MODULES


EXPERT_MODULES = {'gate_proj', 'up_proj', 'down_proj'}


class TargetModulesTest(unittest.TestCase):
    """Targeting expert projections multiplies adapter size by expert count."""

    def test_expert_projections_are_excluded(self) -> None:
        self.assertFalse(EXPERT_MODULES.intersection(TARGET_MODULES))

    def test_attention_projections_are_included(self) -> None:
        for name in ('q_proj', 'k_proj', 'v_proj', 'o_proj'):
            self.assertIn(name, TARGET_MODULES)


if __name__ == '__main__':
    unittest.main()
