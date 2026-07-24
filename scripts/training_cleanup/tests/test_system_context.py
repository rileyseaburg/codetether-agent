"""System context preservation tests."""

import unittest

from fixtures import jsonl, record

from training_cleanup.correlation import pairs
from training_cleanup.group_clean import clean
from training_cleanup.prepare_object import build as prepare


class SystemContextTests(unittest.TestCase):
    """Verify pre-prompt instructions remain in accepted samples."""

    def test_system_prefix_is_attached_to_first_user_turn(self) -> None:
        rows = [
            record('system', 'agent_message', 1, content='rules'),
            record('user', 'user_prompt', 2, content='inspect'),
            record('assistant', 'agent_message', 3, content='done'),
        ]
        prepared = prepare(('s3a://bucket/source', jsonl(rows)), 32_768)
        keyed = list(pairs(prepared))
        result = clean((keyed[0][0], [value for _, value in keyed]), 96, 65_536)
        self.assertEqual(len(result.samples), 1)
        roles = [item['role'] for item in result.samples[0]['messages']]
        self.assertEqual(roles, ['system', 'user', 'assistant'])
        self.assertEqual(result.rejected, [])


if __name__ == '__main__':
    unittest.main()
