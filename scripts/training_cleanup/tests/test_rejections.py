"""Malformed and sensitive-source rejection tests."""

import unittest

from fixtures import jsonl, record

from training_cleanup.clean import object_text
from training_cleanup.quarantine_rows import build


class RejectionTests(unittest.TestCase):
    """Verify bad rows never enter accepted samples."""

    def test_unpaired_tool_call_is_quarantined(self) -> None:
        """Reject an assistant tool call with no corresponding result."""
        call = {
            'id': 'missing',
            'type': 'function',
            'function': {'name': 'x', 'arguments': '{}'},
        }
        values = [record('user', 'user_prompt', 1, content='go')]
        values.append(
            record(
                'assistant',
                'tool_request_batch',
                2,
                content=None,
                tool_calls=[call],
            )
        )
        result = object_text('source', jsonl(values), 32_768)
        self.assertFalse(result.samples)
        self.assertIn(
            'unpaired_or_duplicate_tool',
            {row.reason for row in result.rejected},
        )

    def test_secret_payload_is_not_copied_to_quarantine(self) -> None:
        """Retain a source pointer without duplicating detected credentials."""
        value = record(
            'user', 'user_prompt', 1, content='api_key=abcdefghijklmnop1234'
        )
        result = object_text('source', jsonl([value]), 32_768)
        rejected = result.rejected[0]
        row = build(rejected, 'source', 'digest', 'run', '2026-07-17T00:00:00Z')
        self.assertEqual(rejected.reason, 'possible_secret')
        self.assertIsNone(row['record_json'])


if __name__ == '__main__':
    unittest.main()
