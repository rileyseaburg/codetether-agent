"""Cross-object tool-pair reconstruction tests."""

import unittest

from fixtures import jsonl, record, tool_pair

from training_cleanup.correlation import pairs
from training_cleanup.group_clean import clean
from training_cleanup.prepare_object import build as prepare


class CrossObjectToolTests(unittest.TestCase):
    """Verify calls and responses can arrive in separate batches."""

    def test_tool_pair_joins_across_objects(self) -> None:
        """Accept one complete tool sample with three source origins."""
        call, response = tool_pair(2)
        values = [
            [record('user', 'user_prompt', 1, content='inspect')],
            [call],
            [response],
        ]
        objects = [
            prepare((f's3a://bucket/{index}', jsonl(rows)), 32_768)
            for index, rows in enumerate(values)
        ]
        keyed = [pair for item in objects for pair in pairs(item)]
        result = clean((keyed[0][0], [row for _, row in keyed]), 96, 65_536)
        self.assertEqual(len(result.samples), 1)
        messages = result.samples[0]['messages']
        call_ids = {call['id'] for call in messages[1]['tool_calls']}
        response_ids = {
            message['tool_call_id']
            for message in messages
            if message['role'] == 'tool'
        }
        self.assertEqual(call_ids, response_ids)
        self.assertEqual(
            len(result.samples[0]['metadata']['source_records']), 3
        )


if __name__ == '__main__':
    unittest.main()
