"""Native per-user MSI sequencing for local package/recognizer setup."""
from xml.etree.ElementTree import Element
from .payload import element

def add_actions(product: Element) -> None:
    element(product, 'CustomAction', Id='ResolveSetupHost', Property='SETUPHOST',
            Value='[System64Folder]WindowsPowerShell\\v1.0\\powershell.exe', Impersonate='yes')
    target = '-NoProfile -ExecutionPolicy RemoteSigned -File "[INSTALLROOT]script\\windows-install\\msi-entry.ps1"'
    element(product, 'CustomAction', Id='ConfigureNativeOcr', Property='SETUPHOST',
            ExeCommand=target, Execute='deferred', Return='check', Impersonate='yes')
    sequence = element(product, 'InstallExecuteSequence')
    element(sequence, 'RemoveExistingProducts', Sequence='1501')
    element(sequence, 'Custom', Action='ResolveSetupHost', Sequence='1001').text = 'NOT REMOVE~="ALL"'
    element(sequence, 'Custom', Action='ConfigureNativeOcr', Sequence='6501').text = (
        'NOT REMOVE~="ALL"'
    )
    element(product, 'Condition', Message='CodeTether requires 64-bit Windows; setup checks Windows 10 build 19041 or later.').text = 'VersionNT64'
    element(product, 'Condition', Message='Inherited AllSigned policy requires signed installer scripts; policy was not changed.').text = (
        'NOT (%PSExecutionPolicyPreference = "AllSigned")')
    element(product, 'Condition', Message='Start this per-user installer normally, not as administrator. It requests Windows consent only when needed.').text = 'Installed OR NOT MsiRunningElevated'
    element(product, 'Condition', Message='First-time CodeTether setup requires an interactive installation for Windows consent.').text = 'Installed OR UILevel >= 3'
    element(product, 'Property', Id='ARPCOMMENTS', Value='Setup cache for the CodeTether MSIX app. Removing this setup cache does not remove the app; uninstall CodeTether separately in Windows Apps.')