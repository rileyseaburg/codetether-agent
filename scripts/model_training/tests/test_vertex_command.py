"""Verify the container command bootstraps before running the entrypoint."""

import unittest

from model_training.vertex_command import BUNDLE, ENTRYPOINT, container_command


class VertexCommandTest(unittest.TestCase):
    """A prebuilt image holds no project code.

    Referencing the entrypoint path directly failed with exit status 127.
    """

    def test_command_runs_through_a_shell(self) -> None:
        command = container_command()
        self.assertEqual(command[:2], ['/bin/bash', '-c'])

    def test_clone_precedes_entrypoint(self) -> None:
        script = container_command()[2]
        self.assertLess(script.index('git clone'), script.index(ENTRYPOINT))

    def test_clone_targets_the_bundle_path(self) -> None:
        script = container_command()[2]
        self.assertIn(BUNDLE, script)

    def test_entrypoint_replaces_the_shell(self) -> None:
        """exec keeps Vertex watching the training process itself."""
        self.assertIn(
            f'exec bash {BUNDLE}/{ENTRYPOINT}', container_command()[2]
        )


if __name__ == '__main__':
    unittest.main()
