"""Verify multi-GPU settings preserve the effective batch size."""

import os
import unittest

from unittest import mock

from model_training.distributed import (
    accumulation_steps,
    is_primary,
    local_rank,
    world_size,
)
from model_training.schedule import ACCUMULATION_STEPS


class DistributedTest(unittest.TestCase):
    """Data parallelism must divide work, not alter optimization semantics."""

    def test_single_gpu_keeps_original_accumulation(self) -> None:
        with mock.patch.dict(os.environ, {}, clear=True):
            self.assertEqual(accumulation_steps(), ACCUMULATION_STEPS)

    def test_eight_gpus_preserve_effective_batch(self) -> None:
        with mock.patch.dict(os.environ, {'WORLD_SIZE': '8'}, clear=True):
            self.assertEqual(accumulation_steps(), 2)
            self.assertEqual(world_size() * accumulation_steps(), 16)

    def test_worker_uses_its_assigned_cuda_device(self) -> None:
        with mock.patch.dict(os.environ, {'LOCAL_RANK': '5'}, clear=True):
            self.assertEqual(local_rank(), 5)

    def test_only_rank_zero_owns_artifacts(self) -> None:
        with mock.patch.dict(os.environ, {'RANK': '0'}, clear=True):
            self.assertTrue(is_primary())
        with mock.patch.dict(os.environ, {'RANK': '3'}, clear=True):
            self.assertFalse(is_primary())


if __name__ == '__main__':
    unittest.main()
