"""Mocked tool selection keeps CI independent of Docker-in-Docker bind mounts."""
import pathlib
import unittest
from unittest.mock import Mock, patch
from .tools import command, native_available

class ToolSelectionTests(unittest.TestCase):
    @patch('windows_msi.tools.shutil.which', return_value='/usr/bin/tool')
    def test_local_tools_do_not_use_docker_mounts(self, lookup: Mock) -> None:
        self.assertTrue(native_available())
        self.assertEqual(command(pathlib.Path('/repo'), 'wixl', ['-v']), ['wixl', '-v'])

    @patch('windows_msi.tools.shutil.which', return_value=None)
    def test_developer_fallback_is_isolated_and_explicit(self, lookup: Mock) -> None:
        args = command(pathlib.Path('/repo'), 'msiinfo', ['suminfo', 'build.msi'])
        self.assertEqual(args[:3], ['docker', 'run', '--rm'])
        self.assertIn('/repo:/workspace:ro', args)
        self.assertIn('none', args)
        writable = command(pathlib.Path('/repo'), 'msibuild', ['build.msi'], writable=True)
        self.assertIn('/repo:/workspace', writable)

if __name__ == '__main__':
    unittest.main()