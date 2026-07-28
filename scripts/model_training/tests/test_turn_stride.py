"""Verify turn sampling keeps the whole corpus by default."""

import os
import unittest

from unittest import mock

from model_training.turn_window import stride


class TurnStrideTest(unittest.TestCase):
    """A cap of 24 discarded roughly 75 percent of assistant turns."""

    def test_uncapped_by_default(self) -> None:
        with mock.patch.dict(os.environ, {}, clear=True):
            self.assertEqual(stride(1000), 1)

    def test_environment_override_caps_turns(self) -> None:
        with mock.patch.dict(os.environ, {'CODETETHER_MAX_TURNS': '24'}):
            self.assertEqual(stride(240), 10)

    def test_short_conversations_are_never_strided(self) -> None:
        with mock.patch.dict(os.environ, {'CODETETHER_MAX_TURNS': '24'}):
            self.assertEqual(stride(10), 1)

    def test_explicit_limit_overrides_environment(self) -> None:
        with mock.patch.dict(os.environ, {'CODETETHER_MAX_TURNS': '24'}):
            self.assertEqual(stride(100, limit=0), 1)


if __name__ == '__main__':
    unittest.main()
