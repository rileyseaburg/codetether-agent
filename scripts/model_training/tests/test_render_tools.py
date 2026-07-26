"""Tool-schema recovery keeps rendered prompts inference-shaped."""

import unittest

from model_training.render_tools import collect


class CollectTest(unittest.TestCase):
    """Verify schema recovery from recorded tool calls."""

    def test_no_tool_calls_yields_no_schemas(self) -> None:
        """Plain chats must render without a tools block."""
        self.assertEqual(collect([{'role': 'user', 'content': 'hi'}]), [])

    def test_recovers_name_and_arguments(self) -> None:
        """Recorded calls define the tool name and argument keys."""
        specs = collect(
            [
                {
                    'role': 'assistant',
                    'tool_calls': [
                        {
                            'function': {
                                'name': 'read',
                                'arguments': '{"path": "src/main.rs"}',
                            }
                        }
                    ],
                }
            ]
        )
        self.assertEqual(len(specs), 1)
        self.assertEqual(specs[0]['function']['name'], 'read')
        self.assertIn('path', specs[0]['function']['parameters']['properties'])


if __name__ == '__main__':
    unittest.main()
