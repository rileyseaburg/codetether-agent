"""Use local MSI tools in CI, or the isolated Docker toolchain on developer hosts."""
import os
import pathlib
import shutil

def native_available() -> bool:
    return all(shutil.which(name) for name in ['wixl', 'msibuild', 'msiinfo', 'msiextract'])

def command(root: pathlib.Path, tool: str, args: list[str], writable: bool = False) -> list[str]:
    if native_available():
        return [tool, *args]
    mount = str(root) + ':/workspace' + ('' if writable else ':ro')
    return ['docker', 'run', '--rm', '--network', 'none',
        '--user', f'{os.getuid()}:{os.getgid()}', '-v', mount,
        '--entrypoint', tool, 'codetether-msi-tools:local', *args]