# Mocked-local prerelease lookup and GNU Windows selection.
param([string]$Root, [string]$Work)
$global:selectedReleaseUri = ''
$global:releaseTagFixture = 'v4.7.6-dev.5'
function Invoke-RestMethod {
    param($Uri, $Headers)
    $global:selectedReleaseUri = $Uri
    $tag = $global:releaseTagFixture
    $base = "https://forgejo.quantum-forge.io/riley/codetether-agent/releases/download/$tag"
    [pscustomobject]@{ tag_name = $tag; prerelease = $true; assets = @(
        [pscustomobject]@{ name = "codetether-$tag-x86_64-pc-windows-gnu.exe"; browser_download_url = "$base/codetether-$tag-x86_64-pc-windows-gnu.exe" },
        [pscustomobject]@{ name = "SHA256SUMS-$tag.txt"; browser_download_url = "$base/SHA256SUMS-$tag.txt" }
    ) }
}
function Invoke-WebRequest {
    param($Uri, $OutFile, [switch]$UseBasicParsing)
    if ($Uri.EndsWith('.txt')) {
        [IO.File]::WriteAllText($OutFile, ('e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855' + "  codetether-$global:releaseTagFixture-x86_64-pc-windows-gnu.exe"))
    } else { [IO.File]::WriteAllBytes($OutFile, [byte[]]@()) }
}
$native = $env:PROCESSOR_ARCHITECTURE; $wow = $env:PROCESSOR_ARCHITEW6432
try {
    $env:PROCESSOR_ARCHITECTURE = 'AMD64'; $env:PROCESSOR_ARCHITEW6432 = ''
    $result = & "$Root/release.ps1" -Work $Work
    Assert-Contract ($result.Exe.EndsWith('x86_64-pc-windows-gnu.exe')) 'GNU executable selected'
    Assert-Contract ($global:selectedReleaseUri.EndsWith('/releases?draft=false&limit=1')) 'latest includes published prereleases'
    $result = & "$Root/release.ps1" -Work $Work -Version 'v4.7.6-dev.5'
    Assert-Contract ($global:selectedReleaseUri.EndsWith('/releases/tags/v4.7.6-dev.5')) 'explicit version remains supported'
    Assert-Throws { & "$Root/release.ps1" -Work $Work -Version '../main' } 'INVALID_RELEASE_TAG'
} finally {
    $env:PROCESSOR_ARCHITECTURE = $native; $env:PROCESSOR_ARCHITEW6432 = $wow
    Remove-Variable selectedReleaseUri, releaseTagFixture -Scope Global
}
