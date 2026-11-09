"""Mocked installer regressions for both release archive layouts and PATH shadowing."""

import io
import os
from pathlib import Path
import tarfile
import tempfile
import unittest
from fixture import ROOT, execute


class InstallerArchives(unittest.TestCase):
    def archive(self, name: str) -> Path:
        root = ROOT / "artifacts/installer-verification"
        root.mkdir(parents=True, exist_ok=True)
        path = Path(tempfile.mkdtemp(prefix="archive-", dir=root)) / "release.tar.gz"
        data = b"#!/bin/sh\nprintf 'codetether 4.7.5\\n'\n"
        with tarfile.open(path, "w:gz") as archive:
            item = tarfile.TarInfo(name)
            item.size, item.mode = len(data), 0o755
            archive.addfile(item, io.BytesIO(data))
        return path

    def test_both_layouts_and_mac_targets(self) -> None:
        for platform in ["x86_64-unknown-linux-gnu", "aarch64-apple-darwin", "x86_64-apple-darwin"]:
            for name in ["codetether", f"codetether-v4.7.5-{platform}"]:
                with self.subTest(platform=platform, name=name):
                    result, case = execute(self.archive(name), platform)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertIn("ok: codetether 4.7.5", result.stdout)
                    self.assertIn("PATH resolves", result.stdout)
                    self.assertTrue(os.access(case / "installed/codetether", os.X_OK))

    def test_missing_binary_preserves_existing_install(self) -> None:
        result, case = execute(self.archive("wrong-file"), "x86_64-unknown-linux-gnu")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("1.0.0", (case / "installed/codetether").read_text())


if __name__ == "__main__":
    unittest.main()