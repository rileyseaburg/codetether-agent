"""Run the shipping installer with retained fixtures and no system/profile writes."""

import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]


def execute(archive: Path, platform: str) -> tuple[subprocess.CompletedProcess[str], Path]:
    evidence = ROOT / "artifacts/installer-verification"
    evidence.mkdir(parents=True, exist_ok=True)
    case = Path(tempfile.mkdtemp(prefix="unix-", dir=evidence))
    source = (ROOT / "install.sh").read_text()
    assert source.rstrip().endswith('main "$@"')
    (case / "functions.sh").write_text(source.rsplit('main "$@"', 1)[0])
    stub = case / "bin"
    stub.mkdir()
    installed = case / "installed"
    installed.mkdir()
    for path in [stub / "codetether", installed / "codetether"]:
        path.write_text("#!/bin/sh\nprintf 'codetether 1.0.0\\n'\n")
        path.chmod(0o755)
    environment = os.environ.copy()
    environment.update(CASE_DIR=str(case), INSTALL_TEST_ARCHIVE=str(archive.resolve()),
                       INSTALL_TEST_PLATFORM=platform, PATH=str(stub) + os.pathsep + environment["PATH"])
    result = subprocess.run(["sh", str(Path(__file__).with_name("harness.sh"))],
                            env=environment, text=True, capture_output=True)
    (case / "installer.log").write_text(result.stdout + result.stderr)
    return result, case