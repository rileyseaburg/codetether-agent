# Install a built Rust companion without running it or changing capture consent.
param([Parameter(Mandatory=$true)][string]$BuildRoot)
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
$source = Join-Path $BuildRoot 'target\debug\codetether-companion.exe'
if (!(Test-Path $source)) { throw 'Built companion executable is missing' }
$folder = Join-Path $env:LOCALAPPDATA 'Programs\CodeTetherCompanion'
$destination = Join-Path $folder 'codetether-companion.exe'
$backup = Join-Path $BuildRoot ('install-backup-' + (Get-Date -Format 'yyyyMMddTHHmmss'))
$running = Get-Process codetether-companion -ErrorAction SilentlyContinue
if ($running | Where-Object { $_.Path -eq $destination }) {
    throw 'Close the installed companion before replacing it; no process was terminated'
}
New-Item -ItemType Directory -Force -Path $folder,$backup | Out-Null
if (Test-Path $destination) { Copy-Item $destination (Join-Path $backup 'codetether-companion.exe') }
Copy-Item $source $destination -Force
$builtHash = (Get-FileHash -Algorithm SHA256 $source).Hash
$installedHash = (Get-FileHash -Algorithm SHA256 $destination).Hash
if ($builtHash -ne $installedHash) { throw 'Installed artifact hash does not match the build' }
$shell = New-Object -ComObject WScript.Shell
$locations = @([Environment]::GetFolderPath('Desktop'), [Environment]::GetFolderPath('Programs'))
$shortcuts = @()
foreach ($location in $locations) {
    $shortcutPath = Join-Path $location 'CodeTether Screen Companion.lnk'
    if (Test-Path $shortcutPath) {
        Copy-Item $shortcutPath (Join-Path $backup ((Split-Path $location -Leaf) + '.lnk'))
    }
    $shortcut = $shell.CreateShortcut($shortcutPath)
    $shortcut.TargetPath = $destination
    $shortcut.WorkingDirectory = $folder
    $shortcut.Description = 'Pair, select a monitor, then Run in background (no taskbar button).'
    $shortcut.Save()
    $shortcuts += $shortcutPath
}
[ordered]@{ installed = $destination; source = $source; sha256 = $installedHash;
    bytes = (Get-Item $destination).Length; backup = $backup; shortcuts = $shortcuts;
    launched = $false; signed = ((Get-AuthenticodeSignature $destination).Status -eq 'Valid')
} | ConvertTo-Json | Set-Content (Join-Path $BuildRoot 'install-receipt.json')
Get-Content (Join-Path $BuildRoot 'install-receipt.json')
