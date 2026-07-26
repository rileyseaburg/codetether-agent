"""Verify assistant-only supervision and short-primer rendering."""

import unittest

from model_training.short_cases import messages
from model_training.turn_split import pairs, text_of


class TurnSplitTest(unittest.TestCase):
    """Only assistant turns may become training completions."""

    def test_only_assistant_turns_are_completions(self) -> None:
        conversation = [
            {'role': 'system', 'content': 'sys'},
            {'role': 'user', 'content': 'hi'},
            {'role': 'assistant', 'content': 'hello'},
            {'role': 'user', 'content': 'again'},
            {'role': 'assistant', 'content': 'sure'},
        ]
        result = pairs(conversation)
        self.assertEqual(len(result), 2)
        self.assertEqual([text_of(r) for _, r in result], ['hello', 'sure'])
        self.assertEqual(result[0][0][-1]['role'], 'user')

    def test_assistant_without_user_context_is_skipped(self) -> None:
        conversation = [
            {'role': 'system', 'content': 'sys'},
            {'role': 'assistant', 'content': 'orphan'},
        ]
        self.assertEqual(pairs(conversation), [])

    def test_short_cases_are_single_turn_pairs(self) -> None:
        for conversation in messages():
            self.assertEqual(len(pairs(conversation)), 1)


if __name__ == '__main__':
    unittest.main()
