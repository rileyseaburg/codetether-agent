# Mocked-local Forgejo manifest, origin and corrupt-download contracts.
param([string]$Root, [string]$Work)
$base = 'https://forgejo.quantum-forge.io/riley/codetether-agent/releases/download/v1.0.0'
$asset = [pscustomobject]@{ name = 'codetether.exe'; browser_download_url = "$base/codetether.exe" }
$manifest = [pscustomobject]@{ name = 'SHA256SUMS-v1.0.0.txt'; browser_download_url = "$base/SHA256SUMS-v1.0.0.txt" }
$release = [pscustomobject]@{ tag_name = 'v1.0.0'; assets = @($asset, $manifest) }
$global:releaseBytes = [Text.Encoding]::UTF8.GetBytes('fixture executable')
$sha = [Security.Cryptography.SHA256]::Create()
try { $expected = ([BitConverter]::ToString($sha.ComputeHash($global:releaseBytes))).Replace('-', '').ToLowerInvariant() }
finally { $sha.Dispose() }
$global:releaseManifest = "$expected  codetether.exe"
function Invoke-WebRequest {
    param($Uri, $OutFile, [switch]$UseBasicParsing)
    if ($Uri.EndsWith('.txt')) { [IO.File]::WriteAllText($OutFile, $global:releaseManifest) }
    else { [IO.File]::WriteAllBytes($OutFile, $global:releaseBytes) }
}
$path = & "$Root/download-asset.ps1" -Asset $asset -Release $release -Work $Work
Assert-Contract ((Get-FileHash $path -Algorithm SHA256).Hash -ieq $expected) 'valid manifest permits the asset'
$global:releaseManifest = ('0' * 64) + '  codetether.exe'
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Release $release -Work $Work } 'RELEASE_INTEGRITY_FAILED'
$global:releaseManifest = "$expected  other.exe"
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Release $release -Work $Work } 'RELEASE_DIGEST_UNAVAILABLE'
$global:releaseManifest = "$expected  codetether.exe`n$expected  codetether.exe"
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Release $release -Work $Work } 'RELEASE_DIGEST_UNAVAILABLE'
$asset.browser_download_url = 'https://github.com/rileyseaburg/codetether-agent/releases/download/v1.0.0/codetether.exe'
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Release $release -Work $Work } 'UNEXPECTED_RELEASE_URL'
$asset.browser_download_url = "$base/codetether.exe"
$manifest.browser_download_url = 'https://example.com/checksums'
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Release $release -Work $Work } 'UNEXPECTED_CHECKSUM_URL'
$manifest.browser_download_url = "$base/SHA256SUMS-v1.0.0.txt"
$release.assets = @($asset)
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Release $release -Work $Work } 'RELEASE_DIGEST_UNAVAILABLE'
$asset.name = '../codetether.exe'
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Release $release -Work $Work } 'UNEXPECTED_RELEASE_URL'
Remove-Variable releaseBytes, releaseManifest -Scope Global

