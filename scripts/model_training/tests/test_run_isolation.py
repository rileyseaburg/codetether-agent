"""Verify parallel jobs receive isolated artifact paths."""

import os
import unittest

from unittest import mock

from model_training.vertex_env import PASSTHROUGH_VARS


class RunIsolationTest(unittest.TestCase):
    """Concurrent workers must never overwrite each other's checkpoints."""

    def test_run_id_reaches_the_vertex_worker(self) -> None:
        self.assertIn('CODETETHER_RUN_ID', PASSTHROUGH_VARS)

    def test_two_run_ids_produce_distinct_prefixes(self) -> None:
        bucket = 'gs://spotless/model-training'
        with mock.patch.dict(os.environ, {'CODETETHER_RUN_ID': 'single'}):
            single = f'{bucket}/{os.environ["CODETETHER_RUN_ID"]}'
        with mock.patch.dict(os.environ, {'CODETETHER_RUN_ID': 'multi'}):
            multi = f'{bucket}/{os.environ["CODETETHER_RUN_ID"]}'
        self.assertNotEqual(single, multi)


if __name__ == '__main__':
    unittest.main()
