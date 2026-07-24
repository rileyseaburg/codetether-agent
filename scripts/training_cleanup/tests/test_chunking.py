"""Tool-safe trainer-sample chunking tests."""

import unittest

from fixtures import jsonl, record, tool_pair

from training_cleanup.correlation import pairs
from training_cleanup.group_clean import clean
from training_cleanup.prepare_object import build as prepare


class ChunkingTests(unittest.TestCase):
    """Verify oversized turns split only between complete tool pairs."""

    def test_tool_pairs_remain_atomic(self) -> None:
        """Repeat the prompt and keep every call beside its result."""
        values = [record('user', 'user_prompt', 1, content='fix')]
        for line in (2, 4, 6):
            values.extend(tool_pair(line))
        prepared = prepare(('s3a://bucket/trace.jsonl', jsonl(values)), 32_768)
        keyed = list(pairs(prepared))
        result = clean((keyed[0][0], [row for _, row in keyed]), 4, 4_096)
        self.assertEqual(len(result.samples), 3)
        for index, sample in enumerate(result.samples):
            messages = sample['messages']
            self.assertEqual(messages[0]['role'], 'user')
            calls = {call['id'] for call in messages[1]['tool_calls']}
            responses = {
                message['tool_call_id']
                for message in messages
                if message['role'] == 'tool'
            }
            self.assertEqual(calls, responses)
            self.assertEqual(sample['metadata']['chunk_index'], index)
            self.assertEqual(sample['metadata']['chunk_count'], 3)


if __name__ == '__main__':
    unittest.main()
