"""Verify artifact surveying reports resumability accurately."""

import tempfile
import unittest

from pathlib import Path

from model_training.run_artifacts import survey


class RunArtifactsTest(unittest.TestCase):
    """Checkpoint ordering decides which state a resume starts from."""

    def test_missing_state_reports_nothing(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            result = survey(Path(directory) / 'absent')
            self.assertFalse(result['state_exists'])
            self.assertFalse(result['log_exists'])
            self.assertEqual(result['checkpoints'], [])

    def test_checkpoints_sort_numerically(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / 'output'
            for step in (250, 1000, 500):
                (output / f'checkpoint-{step}').mkdir(parents=True)
            result = survey(Path(directory))
            self.assertEqual(result['latest_checkpoint'], 'checkpoint-1000')

    def test_final_adapter_is_detected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            (Path(directory) / 'output' / 'final-adapter').mkdir(parents=True)
            self.assertTrue(survey(Path(directory))['final_adapter'])

    def test_log_size_is_reported(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            logs = Path(directory) / 'logs'
            logs.mkdir(parents=True)
            (logs / 'train.log').write_text('step 1\n')
            result = survey(Path(directory))
            self.assertTrue(result['log_exists'])
            self.assertEqual(result['log_bytes'], 7)


if __name__ == '__main__':
    unittest.main()
