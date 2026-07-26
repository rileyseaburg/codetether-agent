"""Verify supervision columns are isolated for TRL."""

import importlib.util
import json
import tempfile
import unittest

from pathlib import Path


HAS_DATASETS = importlib.util.find_spec('datasets') is not None

if HAS_DATASETS:
    from model_training.data_loader import splits


@unittest.skipUnless(HAS_DATASETS, 'datasets is a GPU-runtime dependency')
class DataLoaderTest(unittest.TestCase):
    """Metadata must not reach the trainer's collator."""

    def test_metadata_column_is_dropped(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'pairs.jsonl'
            record = {
                'prompt': 'p',
                'completion': 'c',
                'metadata': {'source': 'test'},
            }
            path.write_text(json.dumps(record) + '\n')
            train, validation = splits(path, path)
            self.assertEqual(
                sorted(train.column_names), ['completion', 'prompt']
            )
            self.assertEqual(
                sorted(validation.column_names), ['completion', 'prompt']
            )

    def test_rows_are_preserved(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'pairs.jsonl'
            rows = [{'prompt': f'p{i}', 'completion': 'c'} for i in range(3)]
            path.write_text(''.join(json.dumps(r) + '\n' for r in rows))
            train, _ = splits(path, path)
            self.assertEqual(len(train), 3)


if __name__ == '__main__':
    unittest.main()
