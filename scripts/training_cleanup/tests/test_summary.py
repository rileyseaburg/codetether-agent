"""Machine-readable run summary tests."""

import json
import unittest

from training_cleanup.run_summary import build
from training_cleanup.settings import Settings


class SummaryTests(unittest.TestCase):
    """Validate quality-gate detail counters."""

    def test_reason_and_quality_histograms_are_included(self) -> None:
        settings = Settings(
            'http://minio',
            'bucket',
            'raw',
            'raw/next',
            'iceberg',
            'codetether',
            'training',
            'run',
            100,
            4,
            False,
        )
        counts = {
            'samples': 2,
            'samples:complete': 2,
            'quarantine': 3,
            'quarantine:heartbeat': 3,
            'manifests': 1,
        }
        result = json.loads(build(settings, counts))
        self.assertEqual(result['quality_tiers'], {'complete': 2})
        self.assertEqual(result['quarantine_reasons'], {'heartbeat': 3})


if __name__ == '__main__':
    unittest.main()
