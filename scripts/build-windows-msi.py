#!/usr/bin/env python3
"""Author and build a real MSI using Linux-hosted Windows Installer tooling."""
import datetime
import hashlib
import json
import pathlib
import subprocess
import shutil
from windows_msi.prepare import prepare
from windows_msi.normalize import normalize
from windows_msi.verify import verify
from windows_msi.tools import command as tool_command

root = pathlib.Path(__file__).resolve().parent.parent
stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
evidence = root / 'artifacts' / 'windows-msi' / stamp
evidence.mkdir(parents=True)
payload, source, version = prepare(root, evidence)
output = evidence / 'codetether-windows.msi'
command = tool_command(root, 'wixl', ['-a', 'x64', '-v', '-o',
    str(output.relative_to(root)), str(source.relative_to(root))], writable=True)
with (evidence / 'build.log').open('w') as log:
    result = subprocess.run(command, cwd=root, stdout=log, stderr=subprocess.STDOUT)
record = {'command': command, 'exit_code': result.returncode, 'version': version,
          'source': str(source), 'log': str(evidence / 'build.log')}
if any(marker in (evidence / 'build.log').read_text() for marker in ['WARNING', 'CRITICAL', 'assertion']):
    record['exit_code'] = 1
    record['error'] = 'MSI authoring diagnostics rejected; output is not approved for distribution'
    result.returncode = 1
if result.returncode == 0:
    try:
        normalize(root, output, evidence)
        verify(root, output, evidence, payload)
    except Exception as error:
        record['exit_code'] = result.returncode = 1
        record['error'] = str(error)
if result.returncode == 0:
    published = root / 'dist' / 'codetether-windows.msi'
    if published.exists():
        shutil.copy2(published, evidence / 'previous-installer.msi')
    pending = published.with_suffix('.msi.pending')
    shutil.copy2(output, pending)
    pending.replace(published)
    record.update(artifact=str(output), bytes=output.stat().st_size,
                  sha256=hashlib.sha256(output.read_bytes()).hexdigest())
(evidence / 'status.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record), flush=True)
if result.returncode:
    print((evidence / 'build.log').read_text(), flush=True)
raise SystemExit(result.returncode)