"""Governed export tests."""

import json
import tempfile
import unittest

from pathlib import Path

from model_training.dataset_file import write
from model_training.export_query import build
from model_training.export_verify import splits


class ExportTests(unittest.TestCase):
    """Verify deterministic serialization and split isolation."""

    def test_query_pins_snapshot_and_split(self) -> None:
        sql = build('validation', 12, 99)
        self.assertIn('FOR VERSION AS OF 99', sql)
        self.assertIn("split_bucket = '0'", sql)
        self.assertIn('LIMIT 12', sql)

    def test_split_verifier_rejects_overlap(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            train = root / 'train.jsonl'
            validation = root / 'validation.jsonl'
            write(train, [_row('train', 'shared')])
            write(validation, [_row('validation', 'shared')])
            with self.assertRaisesRegex(ValueError, 'correlation leakage'):
                splits(train, validation)


def _row(sample: str, correlation: str) -> dict[str, object]:
    messages = [
        {'role': 'user', 'content': 'question'},
        {'role': 'assistant', 'content': 'answer'},
    ]
    return {
        'sample_id': sample,
        'correlation_id': correlation,
        'message_sha': sample,
        'messages_json': json.dumps(messages),
    }


if __name__ == '__main__':
    unittest.main()
