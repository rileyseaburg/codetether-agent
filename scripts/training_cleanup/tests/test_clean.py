"""Conversation reconstruction and rejection tests."""

import unittest

from collections import Counter

from fixtures import jsonl, record, tool_pair

from training_cleanup.clean import object_text


class CleanTests(unittest.TestCase):
    """Verify records become safe trainer-facing conversations."""

    def test_observed_thirteen_record_shape(self) -> None:
        """Drop duplicate telemetry but retain four tool-call pairs."""
        values = [
            record('user', 'user_prompt', 1, content='fix', correlation_id=None)
        ]
        values += [
            record('tool', 'tool_output_full', line, content='noise')
            for line in range(2, 6)
        ]
        for line in (6, 8, 10, 12):
            values += tool_pair(line)
        result = object_text('s3a://bucket/batch.jsonl', jsonl(values), 32_768)
        self.assertEqual(len(result.samples), 1)
        self.assertEqual(result.accepted_records, 9)
        self.assertEqual(len(result.samples[0]['messages']), 9)
        self.assertEqual(
            result.samples[0]['metadata']['quality_tier'], 'tool_use'
        )
        self.assertEqual(
            Counter(row.reason for row in result.rejected),
            {'duplicate_tool_output': 4},
        )

    def test_visible_assistant_makes_complete_sample(self) -> None:
        """Classify a turn with a final assistant response as complete."""
        values = [record('user', 'user_prompt', 1, content='hello')]
        values.append(record('assistant', 'agent_message', 2, content='hi'))
        result = object_text('source', jsonl(values), 32_768)
        self.assertEqual(
            result.samples[0]['metadata']['quality_tier'], 'complete'
        )


if __name__ == '__main__':
    unittest.main()
