"""Build reviewable WiX source for a real per-user MSI bootstrap package."""
import hashlib
import pathlib
import uuid
import xml.etree.ElementTree as ET
from .actions import add_actions
from .payload import NS, add_files, element

def author(payload: pathlib.Path, version: str, output: pathlib.Path) -> None:
    digest = hashlib.sha256()
    for path in sorted(payload.rglob('*')):
        if path.is_file():
            digest.update(path.relative_to(payload).as_posix().encode())
            digest.update(path.read_bytes())
    ET.register_namespace('', NS)
    document = ET.Element('{' + NS + '}Wix')
    product = element(document, 'Product',
        Id=str(uuid.uuid5(uuid.NAMESPACE_URL, 'codetether/payload/' + digest.hexdigest())),
        Name='CodeTether Setup ' + version, Language='1033', Version=version.split('-')[0],
        Manufacturer='CodeTether', UpgradeCode='861998d0-e0d5-4f71-9d05-a8af852dac42')
    element(product, 'Package', Id='*', InstallerVersion='500', Compressed='yes',
        InstallScope='perUser',
        Description='CodeTether native OCR and shadow input setup')
    element(product, 'Media', Id='1', Cabinet='payload.cab', EmbedCab='yes')
    upgrade = element(product, 'Upgrade', Id='861998d0-e0d5-4f71-9d05-a8af852dac42')
    element(upgrade, 'UpgradeVersion', Minimum='0.0.0', IncludeMinimum='yes',
        Maximum=version.split('-')[0], IncludeMaximum='yes', Property='PREVIOUSVERSIONS')
    element(upgrade, 'UpgradeVersion', Minimum=version.split('-')[0],
        IncludeMinimum='no', OnlyDetect='yes', Property='NEWERVERSION')
    element(product, 'Condition', Message='A newer CodeTether installer is already present.').text = 'Installed OR NOT NEWERVERSION'
    add_files(product, payload)
    add_actions(product)
    output.parent.mkdir(parents=True, exist_ok=True)
    ET.indent(document)
    ET.ElementTree(document).write(output, encoding='utf-8', xml_declaration=True)