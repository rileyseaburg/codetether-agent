"""Reject MSI files that omit setup, run it out of order, or request SYSTEM context."""
import hashlib
import json
import pathlib
import subprocess
from .inspection import inspect
from .table_rules import validate
from .tools import command as tool_command

def verify(root: pathlib.Path, output: pathlib.Path, evidence: pathlib.Path, payload: pathlib.Path) -> None:
    tables = inspect(root, output, evidence)
    validate(tables)
    extracted = evidence / 'extracted'
    extracted.mkdir()
    command = tool_command(root, 'msiextract',
        ['-C', str(extracted.relative_to(root)), str(output.relative_to(root))], writable=True)
    subprocess.run(command, cwd=root, check=True, stdout=subprocess.DEVNULL)
    matches = list(extracted.rglob('codetether.exe')); assert len(matches) == 1
    count = 0
    for path in payload.rglob('*'):
        if path.is_file():
            installed = matches[0].parent / path.relative_to(payload)
            assert hashlib.sha256(installed.read_bytes()).digest() == hashlib.sha256(path.read_bytes()).digest(), path
            count += 1
    (evidence / 'verification.json').write_text(json.dumps({'level':'static/local',
        'payload_files_verified':count,'tables_verified':True,'windows_execution':'not-run'}, indent=2)+'\n')