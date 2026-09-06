"""Author MSI file components for the self-contained local installer payload."""
import hashlib
import pathlib
import uuid
import xml.etree.ElementTree as ET

NS = 'http://schemas.microsoft.com/wix/2006/wi'

def element(parent: ET.Element, kind: str, **attributes: str) -> ET.Element:
    return ET.SubElement(parent, '{' + NS + '}' + kind, attributes)

def identifier(prefix: str, name: str) -> str:
    return prefix + hashlib.sha256(name.encode()).hexdigest()[:24]

def add_files(product: ET.Element, payload: pathlib.Path) -> None:
    target = element(product, 'Directory', Id='TARGETDIR', Name='SourceDir')
    local = element(target, 'Directory', Id='LocalAppDataFolder')
    app = element(local, 'Directory', Id='CodeTetherFolder', Name='CodeTether')
    install = element(app, 'Directory', Id='INSTALLROOT', Name='InstallerBootstrap')
    directories = {pathlib.PurePosixPath('.'): install}
    feature = element(product, 'Feature', Id='Bootstrap', Title='CodeTether', Level='1')
    for path in sorted(payload.rglob('*')):
        if not path.is_file():
            continue
        relative = pathlib.PurePosixPath(path.relative_to(payload).as_posix())
        directory = relative.parent
        chain = list(reversed(directory.parents)) + [directory]
        for item in chain:
            if item not in directories:
                directories[item] = element(directories[item.parent], 'Directory',
                    Id=identifier('D', str(item)), Name=item.name)
        component_id = identifier('C', str(relative))
        component = element(directories[directory], 'Component', Id=component_id,
            Guid=str(uuid.uuid5(uuid.NAMESPACE_URL, 'codetether/msi/' + str(relative))), Win64='yes')
        element(component, 'File', Id=identifier('F', str(relative)),
            Name=relative.name, Source=str(path.relative_to(pathlib.Path(__file__).resolve().parents[2])), KeyPath='yes')
        element(feature, 'ComponentRef', Id=component_id)