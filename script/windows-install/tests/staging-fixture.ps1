# Fixture bytes are deliberately not a usable installer or executable.
param([string]$Root, [string]$Directory)
$helpers = Join-Path $Directory 'script/windows-install'
New-Item -ItemType Directory -Path $helpers -Force | Out-Null
$names = @('alias-tag', 'build-package', 'bundled-source', 'certificate', 'desktop-host',
    'download-asset', 'elevate', 'entry', 'executable', 'functiongemma', 'inspect-package',
    'key-policy', 'manifest', 'ocr-capabilities', 'preserve-package', 'probe-ocr',
    'publish-alias', 'register', 'release', 'resolve-alias', 'run-ocr-probe', 'sdk', 'trust-leaf', 'unpack',
    'setup-vault', 'vault-settings', 'vault-token', 'vault-dialog', 'save-vault', 'extra-helper')
foreach ($name in $names) { Set-Content -LiteralPath (Join-Path $helpers "$name.ps1") -Value "# fixture $name" }
Copy-Item -LiteralPath (Join-Path $Root 'stage-bundle.ps1') -Destination $helpers
[IO.File]::WriteAllBytes((Join-Path $Directory 'codetether.exe'), [byte[]]@(0, 1, 127, 128, 255))
[IO.File]::WriteAllBytes((Join-Path $Directory 'runtime.dll'), [byte[]]@(255, 42, 0, 128))
Set-Content -LiteralPath (Join-Path $Directory 'README.md') -Value 'must not be copied'
New-Item -ItemType Directory -Path (Join-Path $helpers 'tests') | Out-Null
Set-Content -LiteralPath (Join-Path $helpers 'tests/never.ps1') -Value "throw 'nested scripts must not be staged'"
Set-Content -LiteralPath (Join-Path $Directory 'install.ps1') -Value @'
param([string]$ExePath)
if (-not $PSScriptRoot.StartsWith($env:LOCALAPPDATA)) { throw 'REMOTE_SCRIPT_EXECUTION' }
if ($ExePath -ne (Join-Path $PSScriptRoot 'codetether.exe')) { throw 'REMOTE_EXE_PATH' }
Set-Content -LiteralPath (Join-Path $PSScriptRoot 'installer-called.txt') -Value $ExePath
'@