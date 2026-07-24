"""Cross-object source-manifest accounting tests."""

import unittest

from fixtures import jsonl, record

from training_cleanup.correlation import pairs
from training_cleanup.group_clean import clean
from training_cleanup.manifest_global import build as manifest
from training_cleanup.outcome_events import sample as sample_events
from training_cleanup.prepare_object import build as prepare


class GlobalManifestTests(unittest.TestCase):
    """Verify every raw object retains auditable line accounting."""

    def test_sample_origins_are_accounted_per_object(self) -> None:
        """Attribute the sample once and accepted records to both objects."""
        objects = [
            prepare(
                (
                    's3a://bucket/one',
                    jsonl([record('user', 'user_prompt', 1, content='hello')]),
                ),
                32_768,
            ),
            prepare(
                (
                    's3a://bucket/two',
                    jsonl(
                        [record('assistant', 'agent_message', 2, content='hi')]
                    ),
                ),
                32_768,
            ),
        ]
        keyed = [pair for item in objects for pair in pairs(item)]
        result = clean((keyed[0][0], [row for _, row in keyed]), 96, 65_536)
        events = list(sample_events(result.samples[0], 'run', 'now'))
        by_uri = {item.source_uri: [] for item in objects}
        for uri, event in events:
            by_uri[uri].append(event)
        rows = [
            manifest(item, by_uri[item.source_uri], 'run', 'now')
            for item in objects
        ]
        self.assertEqual([row['accepted_records'] for row in rows], [1, 1])
        self.assertEqual([row['accepted_samples'] for row in rows], [1, 0])


if __name__ == '__main__':
    unittest.main()
