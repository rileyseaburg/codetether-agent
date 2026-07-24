"""Stable Iceberg merge identity tests."""

import unittest

from training_cleanup.merge_sql import statement


class MergeTests(unittest.TestCase):
    """Prevent recurring cleanup runs from duplicating historical rows."""

    def test_merge_uses_stable_row_key_across_runs(self) -> None:
        sql = statement('catalog.ns.samples', 'incoming', 'sample_id')
        self.assertIn('target.sample_id = source.sample_id', sql)
        self.assertNotIn('target.run_id', sql)

    def test_manifest_can_upgrade_cleanup_version(self) -> None:
        """Replace an older manifest while retaining its stable source key."""
        sql = statement('catalog.ns.manifests', 'incoming', 'source_uri', True)
        self.assertIn('source.cleanup_version > target.cleanup_version', sql)
        self.assertIn('THEN UPDATE SET *', sql)


if __name__ == '__main__':
    unittest.main()
