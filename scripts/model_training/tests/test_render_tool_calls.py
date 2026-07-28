"""Verify assistant tool calls reach the supervised completion.

Sampling 6,000 v4 pairs found tool calls in 4,989 prompts and 0 completions,
so loss never covered an emitted call even though ingestion recorded 9,943
assistant messages carrying `tool_calls`.
"""

import unittest

from model_training.render_tool_calls import render_calls
from model_training.turn_split import text_of


CALL = {
    'function': {
        'name': 'bash',
        'arguments': {'command': 'uv run pytest -q'},
    }
}
STRING_ARGS = {'function': {'name': 'read', 'arguments': '{"path": "a.py"}'}}


class RenderToolCallsTest(unittest.TestCase):
    """Completions must carry calls in Qwen3.5 native syntax."""

    def test_call_is_rendered_in_native_syntax(self) -> None:
        text = render_calls([CALL])
        self.assertIn('<tool_call>', text)
        self.assertIn('<function=bash>', text)
        self.assertIn('<parameter=command>', text)
        self.assertIn('uv run pytest -q', text)

    def test_json_encoded_arguments_are_parsed(self) -> None:
        self.assertIn('<parameter=path>', render_calls([STRING_ARGS]))

    def test_completion_includes_prose_and_call(self) -> None:
        message = {
            'role': 'assistant',
            'content': 'Running tests.',
            'tool_calls': [CALL],
        }
        text = text_of(message)
        self.assertIn('Running tests.', text)
        self.assertIn('<function=bash>', text)

    def test_call_only_turn_still_supervised(self) -> None:
        message = {'role': 'assistant', 'content': '', 'tool_calls': [CALL]}
        self.assertIn('<function=bash>', text_of(message))

    def test_prose_only_turn_is_unchanged(self) -> None:
        message = {'role': 'assistant', 'content': 'Done.'}
        self.assertEqual(text_of(message), 'Done.')

    def test_malformed_call_is_skipped(self) -> None:
        self.assertEqual(render_calls([{'function': {}}]), '')


if __name__ == '__main__':
    unittest.main()
