"""Export real MSI tables and preserve their compiler-independent evidence."""
import csv
import pathlib
import subprocess
from .tools import command as tool_command

def inspect(root: pathlib.Path, output: pathlib.Path, evidence: pathlib.Path) -> dict:
    tables = {}
    for name in ['CustomAction', 'InstallExecuteSequence', 'Property', 'Upgrade', 'File', 'Directory', 'Component']:
        command = tool_command(root, 'msiinfo',
            ['export', str(output.relative_to(root)), name])
        text = subprocess.check_output(command, cwd=root, text=True)
        (evidence / f'{name}.idt').write_text(text)
        lines = text.splitlines()
        tables[name] = list(csv.DictReader(lines[3:], fieldnames=lines[0].split('\t'), delimiter='\t'))
    command = tool_command(root, 'msiinfo', ['suminfo', str(output.relative_to(root))])
    summary = subprocess.check_output(command, cwd=root, text=True)
    (evidence / 'summary.txt').write_text(summary)
    tables['summary'] = summary
    return tables