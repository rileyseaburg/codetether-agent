# Copy trusted bundle bytes only; the launcher unblocks and executes local copies.
# SHA256 proves copy integrity, not publisher authenticity. Stages are never deleted.
param([Parameter(Mandatory)][string]$Source, [Parameter(Mandatory)][string]$Destination)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$helperPath = 'script/windows-install'
$required = @('alias-tag', 'build-package', 'bundled-source', 'certificate', 'desktop-host',
    'download-asset', 'elevate', 'entry', 'executable', 'functiongemma', 'inspect-package',
    'key-policy', 'manifest', 'ocr-capabilities', 'preserve-package', 'probe-ocr',
    'publish-alias', 'register', 'release', 'resolve-alias', 'run-ocr-probe', 'sdk', 'stage-bundle',
    'trust-leaf', 'unpack', 'setup-vault', 'vault-settings', 'vault-token',
    'vault-dialog', 'save-vault') | ForEach-Object { "$helperPath/$_.ps1" }
$required = @('install.ps1', 'codetether.exe') + @($required)
foreach ($relative in $required) {
    if (-not (Test-Path -LiteralPath (Join-Path $Source $relative) -PathType Leaf)) {
        throw "BUNDLE_INCOMPLETE: Missing $relative; extract the entire ZIP."
    }
}
$files = @('install.ps1', 'codetether.exe')
$files += @(Get-ChildItem -LiteralPath (Join-Path $Source $helperPath) -Filter '*.ps1' -File | ForEach-Object { "$helperPath/$($_.Name)" })
$files += @(Get-ChildItem -LiteralPath $Source -Filter '*.dll' -File | ForEach-Object Name)
$manifest = @()
try {
    foreach ($relative in $files) {
        $from = Join-Path $Source $relative; $to = Join-Path $Destination $relative
        $expected = (Get-FileHash -LiteralPath $from -Algorithm SHA256).Hash
        New-Item -ItemType Directory -Path (Split-Path $to) -Force | Out-Null
        # The bootstrap has already been copied: verify rather than replace it.
        if (-not (Test-Path -LiteralPath $to)) { Copy-Item -LiteralPath $from -Destination $to }
        $actual = (Get-FileHash -LiteralPath $to -Algorithm SHA256).Hash
        $manifest += [pscustomobject]@{ Path = $relative; SourceSHA256 = $expected; LocalSHA256 = $actual }
        if ($actual -ne $expected) { throw "BUNDLE_INTEGRITY_FAILED: $relative" }
    }
} finally {
    ConvertTo-Json -InputObject @($manifest) -Depth 3 | Set-Content -LiteralPath (Join-Path $Destination 'bundle-hashes.json')
}
Write-Host "Local staging verified $($manifest.Count) files; SHA256 evidence: $Destination/bundle-hashes.json"