"""Verify probes advertise tools so tool-call rate is observable.

Scoring bare prompts reported `tool_call_rate: 0.0` at steps 0, 100, 200,
and 300 because Qwen's chat template omits the tool-call instruction block
when no schema is passed. The metric was unobservable, not zero.
"""

import unittest

from model_training.behaviour_probe import CALL_MARKERS
from model_training.probe_tools import TOOL_SCHEMA


class ProbeToolsTest(unittest.TestCase):
    """The gate's primary criterion must be measurable."""

    def test_schema_is_not_empty(self) -> None:
        self.assertTrue(TOOL_SCHEMA)

    def test_every_entry_is_a_callable_function(self) -> None:
        for entry in TOOL_SCHEMA:
            self.assertEqual(entry['type'], 'function')
            function = entry['function']
            self.assertIn('name', function)
            self.assertIn('parameters', function)

    def test_markers_cover_the_qwen_function_syntax(self) -> None:
        """Qwen3.5 nests `<function=name>` inside `<tool_call>`."""
        self.assertIn('<tool_call>', CALL_MARKERS)
        self.assertIn('<function=', CALL_MARKERS)


if __name__ == '__main__':
    unittest.main()
