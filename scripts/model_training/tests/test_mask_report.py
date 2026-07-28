"""Verify the masking audit detects sequence-length regressions."""

import json
import tempfile
import unittest

from pathlib import Path

from model_training.mask_report import audit


class _Tokenizer:
    """Whitespace tokenizer standing in for the real one."""

    def __call__(
        self, text: str, add_special_tokens: bool = True
    ) -> dict[str, list[int]]:
        return {'input_ids': [0] * len(text.split())}


class MaskReportTest(unittest.TestCase):
    """A window smaller than the prompt destroys all supervision."""

    def _pairs(self, directory: str) -> Path:
        path = Path(directory) / 'pairs.jsonl'
        record = {
            'prompt': ' '.join(['p'] * 100),
            'completion': ' '.join(['c'] * 10),
        }
        path.write_text(json.dumps(record) + '\n')
        return path

    def test_generous_window_keeps_supervision(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            report = audit(self._pairs(directory), _Tokenizer(), 10, 8192)
            self.assertEqual(report['fully_masked'], 0)
            self.assertGreater(report['supervised_fraction'], 0)

    def test_window_below_prompt_length_is_fully_masked(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            report = audit(self._pairs(directory), _Tokenizer(), 10, 50)
            self.assertEqual(report['fully_masked'], 1)


if __name__ == '__main__':
    unittest.main()
