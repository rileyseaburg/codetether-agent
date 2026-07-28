"""Verify the Vertex worker pool specification is well formed."""

import unittest

from model_training.vertex_spec import BOOT_DISK_GB, JobRequest, build


class VertexSpecTest(unittest.TestCase):
    """A malformed spec fails only after submission, so check it locally."""

    def _spec(self, preemptible: bool = False) -> dict[str, object]:
        return build(
            JobRequest(
                image='gcr.io/p/trainer:1',
                machine='a2-ultragpu-1g',
                accelerator='NVIDIA_A100_80GB',
                environment={'VAULT_ADDR': 'https://vault.example'},
                preemptible=preemptible,
            )
        )

    def test_single_worker_pool(self) -> None:
        pools = self._spec()['workerPoolSpecs']
        self.assertEqual(len(pools), 1)
        self.assertEqual(pools[0]['replicaCount'], 1)

    def test_disk_holds_base_model_and_checkpoints(self) -> None:
        pool = self._spec()['workerPoolSpecs'][0]
        self.assertGreaterEqual(pool['diskSpec']['bootDiskSizeGb'], 200)
        self.assertEqual(pool['diskSpec']['bootDiskSizeGb'], BOOT_DISK_GB)

    def test_environment_is_rendered_as_name_value_pairs(self) -> None:
        env = self._spec()['workerPoolSpecs'][0]['containerSpec']['env']
        self.assertEqual(
            env, [{'name': 'VAULT_ADDR', 'value': 'https://vault.example'}]
        )

    def test_preemptible_requests_spot_with_restart(self) -> None:
        """Non-preemptible A100 40 GB quota is zero on this project."""
        scheduling = self._spec(preemptible=True)['scheduling']
        self.assertEqual(scheduling['strategy'], 'SPOT')
        self.assertTrue(scheduling['restartJobOnWorkerRestart'])

    def test_on_demand_omits_scheduling(self) -> None:
        self.assertNotIn('scheduling', self._spec())


if __name__ == '__main__':
    unittest.main()
