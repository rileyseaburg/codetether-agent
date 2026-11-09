"""Normalize version-bound inclusivity omitted by msitools 0.101's WiX compiler."""
import pathlib
import subprocess
import csv
import io
from .tools import command as tool_command

def normalize(root: pathlib.Path, output: pathlib.Path, evidence: pathlib.Path) -> None:
    # msitools 0.101 emits only VersionMinInclusive for IncludeMaximum=yes.
    # Make same-core-version development upgrades explicit in the MSI database.
    text = subprocess.check_output(tool_command(root, 'msiinfo',
        ['export', str(output.relative_to(root)), 'Upgrade']), cwd=root, text=True)
    (evidence / 'Upgrade-original.idt').write_text(text)
    rows = list(csv.reader(io.StringIO(text), delimiter='\t'))
    for row in rows[3:]:
        if row[-1] == 'PREVIOUSVERSIONS':
            row[4] = '768'
    normalized = evidence / 'Upgrade.idt'
    with normalized.open('w', newline='') as stream:
        csv.writer(stream, delimiter='\t', lineterminator='\r\n').writerows(rows)
    command = tool_command(root, 'msibuild',
        [str(output.relative_to(root)), '-i', str(normalized.relative_to(root))], writable=True)
    with (evidence / 'normalization.log').open('w') as log:
        subprocess.run(command, cwd=root, stdout=log, stderr=subprocess.STDOUT, check=True)