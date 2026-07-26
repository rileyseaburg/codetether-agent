"""Verify a process with a deleted working directory is reported wedged."""

import os
import subprocess
import tempfile
import unittest

from pathlib import Path

from model_training.proc_probe import describe


@unittest.skipUnless(Path('/proc').is_dir(), 'requires procfs')
class ProcProbeTest(unittest.TestCase):
    """Deleting a running process's directory must be detectable."""

    def test_live_process_is_healthy(self) -> None:
        state = describe(os.getpid())
        self.assertTrue(state['running'])
        self.assertFalse(state['deleted_cwd'])

    def test_missing_process_is_not_running(self) -> None:
        self.assertFalse(describe(999_999)['running'])

    def test_deleted_working_directory_is_detected(self) -> None:
        parent = tempfile.mkdtemp()
        victim = Path(parent) / 'gone'
        victim.mkdir()
        child = subprocess.Popen(['sleep', '30'], cwd=victim)
        try:
            victim.rmdir()
            state = describe(child.pid)
            self.assertTrue(state['running'])
            self.assertTrue(state['deleted_cwd'])
            self.assertTrue(state['deleted_state'])
        finally:
            child.kill()
            child.wait()


if __name__ == '__main__':
    unittest.main()
