"""Executor-side source-object decoding tests."""

import unittest

from unittest.mock import MagicMock, patch

from training_cleanup.s3_reader import partition


class SourceTests(unittest.TestCase):
    """Validate whole-object decoding before JSONL parsing."""

    def test_invalid_utf8_is_retained_for_quarantine(self) -> None:
        body = MagicMock()
        body.read.return_value = b'{"x":"\xff"}'
        client = MagicMock()
        client.get_object.return_value = {'Body': body}
        with patch('training_cleanup.s3_reader.create', return_value=client):
            uri, text = next(
                partition(['example.jsonl'], 'http://minio', 'bucket')
            )
        self.assertEqual(uri, 's3a://bucket/example.jsonl')
        self.assertIn('\ufffd', text)


if __name__ == '__main__':
    unittest.main()
