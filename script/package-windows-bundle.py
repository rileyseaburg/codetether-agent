#!/usr/bin/env python3
"""Bundle the Windows executable with its automatic native-OCR installer."""
import hashlib
import argparse
import json
import pathlib
import shutil
import zipfile

root = pathlib.Path(__file__).resolve().parent.parent
dist = root / 'dist'
parser = argparse.ArgumentParser(description=__doc__)
for name, default in [('binary', dist / 'codetether.exe'), ('archive', dist / 'codetether-windows.zip'), ('staging', dist / 'windows')]:
    parser.add_argument('--' + name, type=pathlib.Path, default=default)
args = parser.parse_args()
destination = args.staging
source = root / 'script' / 'windows-install'
helpers = sorted(source.glob('*.ps1'))
if not args.binary.is_file() or not (source / 'entry.ps1').is_file():
    raise SystemExit('Windows binary and complete installer helpers are required')
files = [(args.binary, pathlib.Path('codetether.exe')),
         (root / 'install.ps1', pathlib.Path('install.ps1')),
         (source / 'README.md', pathlib.Path('README.md'))]
files.extend((path, pathlib.Path('script/windows-install') / path.name) for path in helpers)
for name in ['Install-CodeTether.cmd']:
    if (source / name).is_file():
        files.append((source / name, pathlib.Path(name)))
archive = args.archive
archive.parent.mkdir(parents=True, exist_ok=True)
partial = archive.with_suffix('.zip.partial')
manifest = []
with zipfile.ZipFile(partial, 'w', compression=zipfile.ZIP_DEFLATED) as bundle:
    for path, relative in files:
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        if path.resolve() != target.resolve():
            shutil.copy2(path, target)
        bundle.write(path, relative.as_posix())
        manifest.append({'path': relative.as_posix(),
                         'sha256': hashlib.sha256(path.read_bytes()).hexdigest()})
partial.replace(archive)
(destination / 'bundle-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print(f'Windows executable and automatic setup bundle: {archive}')