"""Verify the trainer configuration passes no duplicate keyword.

A dict expansion supplied `packing` while the call also set it explicitly,
which raised `TypeError: got multiple values for keyword argument 'packing'`
only after a GPU had been provisioned and dependencies installed. Parsing
the source catches it in milliseconds instead.
"""

import ast
import unittest

from pathlib import Path

from model_training.throughput import settings


SOURCE = Path(__file__).resolve().parents[1] / 'trainer_config.py'


class TrainerKwargsTest(unittest.TestCase):
    """Static analysis guards a failure that only appeared on hardware."""

    def _call(self) -> ast.Call:
        tree = ast.parse(SOURCE.read_text())
        for node in ast.walk(tree):
            is_call = isinstance(node, ast.Call)
            if is_call and getattr(node.func, 'id', '') == 'SFTConfig':
                return node
        raise AssertionError('SFTConfig call not found')

    def test_no_duplicate_keywords(self) -> None:
        names = [k.arg for k in self._call().keywords if k.arg]
        duplicates = {name for name in names if names.count(name) > 1}
        self.assertEqual(duplicates, set())

    def test_throughput_keys_are_not_also_explicit(self) -> None:
        """Keys provided by dict expansion must not be repeated."""
        expanded = set(settings(flash_attention=False))
        explicit = {k.arg for k in self._call().keywords if k.arg}
        self.assertEqual(expanded & explicit, set())


if __name__ == '__main__':
    unittest.main()
