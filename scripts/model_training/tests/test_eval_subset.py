"""Verify evaluation is bounded so long runs stay affordable."""

import importlib.util
import json
import tempfile
import unittest

from pathlib import Path

from model_training.constants import MAX_EVAL_SAMPLES


HAS_DATASETS = importlib.util.find_spec('datasets') is not None

if HAS_DATASETS:
    from model_training.data_loader import splits


@unittest.skipUnless(HAS_DATASETS, 'datasets is a GPU-runtime dependency')
class EvalSubsetTest(unittest.TestCase):
    """A full validation pass cost 19 minutes and ran every 250 steps."""

    def test_validation_is_capped(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'pairs.jsonl'
            rows = [
                {'prompt': f'p{i}', 'completion': 'c'}
                for i in range(MAX_EVAL_SAMPLES + 250)
            ]
            path.write_text(''.join(json.dumps(r) + '\n' for r in rows))
            train, validation = splits(path, path)
            self.assertEqual(len(validation), MAX_EVAL_SAMPLES)
            self.assertEqual(len(train), MAX_EVAL_SAMPLES + 250)


if __name__ == '__main__':
    unittest.main()
