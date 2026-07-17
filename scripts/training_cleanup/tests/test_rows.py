"""Typed Iceberg-row transformation tests."""

import json
import unittest

from fixtures import jsonl, record

from training_cleanup.spark_transform import object_lines


class RowTests(unittest.TestCase):
    """Verify Spark receives stable typed table rows."""

    def test_transform_tags_rows_and_manifest(self) -> None:
        """Emit one sample and one manifest with the supplied run ID."""
        source = jsonl(
            [
                record('user', 'user_prompt', 1, content='hello'),
                record('assistant', 'agent_message', 2, content='hi'),
            ]
        )
        output = list(
            object_lines(('s3a://bucket/file.jsonl', source), 32_768, 'run-1')
        )
        self.assertEqual([kind for kind, _ in output], ['samples', 'manifests'])
        sample = json.loads(output[0][1])
        manifest = json.loads(output[1][1])
        self.assertEqual(sample['run_id'], 'run-1')
        self.assertEqual(sample['quality_tier'], 'complete')
        self.assertEqual(manifest['accepted_records'], 2)
        self.assertEqual(len(sample['sample_id']), 64)


if __name__ == '__main__':
    unittest.main()
