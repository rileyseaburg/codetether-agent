# Prefer a signed release MSIX; only absent MSIX assets select executable packaging.
param([string]$Version, [string]$Work)
$repo = 'riley/codetether-agent'
$headers = @{ 'User-Agent' = 'codetether-installer' }
$endpoint = '?draft=false&limit=1'
if ($Version) {
    if ($Version -notmatch '^v?\d+\.\d+\.\d+[-.A-Za-z0-9]*$') { throw 'INVALID_RELEASE_TAG' }
    $endpoint = "/tags/$Version"
}
$release = @(Invoke-RestMethod "https://forgejo.quantum-forge.io/api/v1/repos/$repo/releases$endpoint" -Headers $headers)[0]
if (-not $release) { throw 'RELEASE_UNAVAILABLE' }
$tag = $release.tag_name
if ($tag -notmatch '^v?\d+\.\d+\.\d+[-.A-Za-z0-9]*$') { throw 'INVALID_RELEASE_TAG' }
$native = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
$arch = switch ($native) { 'AMD64' { 'x86_64' } 'ARM64' { 'aarch64' } default { throw "UNSUPPORTED_ARCHITECTURE: $native" } }
$names = @()
foreach ($extension in @('msix', 'zip', 'exe', 'tar.gz')) {
    foreach ($target in @('msvc', 'gnu')) { $names += "codetether-$tag-$arch-pc-windows-$target.$extension" }
}
$asset = $null
foreach ($name in $names) {
    $asset = $release.assets | Where-Object { $_.name -ceq $name } | Select-Object -First 1
    if ($asset) { break }
}
if (-not $asset) { throw 'RELEASE_ASSET_UNAVAILABLE: No compatible Windows release asset. Supply -ExePath for a local build.' }
$path = & "$PSScriptRoot\download-asset.ps1" -Asset $asset -Release $release -Work $Work
if ($asset.name.EndsWith('.msix')) { return @{ Msix = $path; Exe = '' } }
$exe = & "$PSScriptRoot\unpack.ps1" -Path $path -Work $Work
@{ Msix = ''; Exe = $exe }