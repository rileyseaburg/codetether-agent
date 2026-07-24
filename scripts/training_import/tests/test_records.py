"""Native session conversion tests."""

import unittest

from training_import.records import session


class RecordTests(unittest.TestCase):
    def test_preserves_tool_pair_and_developer_context(self) -> None:
        value = {
            'id': 's1',
            'created_at': '2026-01-02T03:04:05.123Z',
            'messages': [
                {'role': 'developer', 'content': [_text('rules')]},
                {'role': 'user', 'content': [_text('inspect')]},
                {
                    'role': 'assistant',
                    'content': [
                        {
                            'type': 'tool_call',
                            'id': 'c1',
                            'name': 'read',
                            'arguments': {'path': 'x'},
                        }
                    ],
                },
                {
                    'role': 'tool',
                    'content': [
                        {
                            'type': 'tool_result',
                            'tool_call_id': 'c1',
                            'content': 'ok',
                        }
                    ],
                },
                {'role': 'assistant', 'content': [_text('done')]},
            ],
        }
        records = session(value)
        roles = ['system', 'user', 'assistant', 'tool', 'assistant']
        self.assertEqual([item['role'] for item in records], roles)
        self.assertEqual(
            records[2]['tool_calls'][0]['id'], records[3]['tool_call_id']
        )
        self.assertEqual(records[3]['name'], 'read')
        self.assertTrue(
            all(item['metadata']['correlation_id'] == 's1' for item in records)
        )


def _text(value: str) -> dict[str, str]:
    return {'type': 'text', 'text': value}
