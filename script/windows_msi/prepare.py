"""Prepare the installer payload, authoring source, and MSI compiler image."""
import pathlib
import subprocess
import re
from .author import author
from .tools import native_available

def prepare(root: pathlib.Path, evidence: pathlib.Path) -> tuple[pathlib.Path, pathlib.Path, str]:
    match = re.search(r'^version\s*=\s*"([^"]+)"', (root / 'Cargo.toml').read_text(), re.MULTILINE)
    if match is None: raise ValueError('Cargo package version is missing')
    version = match.group(1)
    payload = evidence / 'payload'
    source = evidence / 'installer.wxs'
    subprocess.run(['python3', 'script/package-windows-bundle.py', '--staging', str(payload),
                    '--archive', str(evidence / 'payload.zip')], cwd=root, check=True)
    author(payload, version, source)
    if not native_available():
        subprocess.run(['docker', 'build', '-f', 'docker/release/msi.Dockerfile',
                        '-t', 'codetether-msi-tools:local', '.'], cwd=root, check=True)
    return payload, source, version