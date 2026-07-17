"""Source-object decoding tests."""

import unittest

from training_cleanup.spark_source import _text_pair


class SourceTests(unittest.TestCase):
    """Validate whole-object decoding before JSONL parsing."""

    def test_invalid_utf8_is_retained_for_quarantine(self) -> None:
        uri, text = _text_pair(
            {'path': 's3a://bucket/example.jsonl', 'content': b'{"x":"\xff"}'}
        )
        self.assertEqual(uri, 's3a://bucket/example.jsonl')
        self.assertIn('\ufffd', text)


if __name__ == '__main__':
    unittest.main()
