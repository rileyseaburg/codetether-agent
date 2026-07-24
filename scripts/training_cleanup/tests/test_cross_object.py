"""Cross-object conversation reconstruction tests."""

import unittest

from fixtures import jsonl, record

from training_cleanup.correlation import pairs
from training_cleanup.group_clean import clean
from training_cleanup.outcome_events import sample as sample_events
from training_cleanup.prepare_object import build as prepare


class CrossObjectTests(unittest.TestCase):
    """Verify one turn can span immutable JSONL objects."""

    def test_prompt_and_target_join_across_objects(self) -> None:
        """Retain both raw origins in one reconstructed sample."""
        first = prepare(
            (
                's3a://bucket/one.jsonl',
                jsonl([record('user', 'user_prompt', 1, content='hello')]),
            ),
            32_768,
        )
        second = prepare(
            (
                's3a://bucket/two.jsonl',
                jsonl([record('assistant', 'agent_message', 2, content='hi')]),
            ),
            32_768,
        )
        keyed = [*pairs(first), *pairs(second)]
        result = clean((keyed[0][0], [row for _, row in keyed]), 96, 65_536)
        self.assertEqual(len(result.samples), 1)
        metadata = result.samples[0]['metadata']
        self.assertEqual(len(metadata['source_records']), 2)
        events = list(sample_events(result.samples[0], 'run', '2026-07-23'))
        self.assertEqual(
            {event[0] for event in events},
            {
                's3a://bucket/one.jsonl',
                's3a://bucket/two.jsonl',
            },
        )


if __name__ == '__main__':
    unittest.main()
