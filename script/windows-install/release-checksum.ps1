# Require exactly one checksum entry from the selected Forgejo release.
param([string]$AssetName, [psobject]$Release, [string]$Work)
$tag = $Release.tag_name
if ($tag -notmatch '^v?\d+\.\d+\.\d+[-.A-Za-z0-9]*$') { throw 'INVALID_RELEASE_TAG' }
$name = "SHA256SUMS-$tag.txt"
$assets = @($Release.assets | Where-Object { $_.name -ceq $name })
if ($assets.Count -ne 1) { throw 'RELEASE_DIGEST_UNAVAILABLE: Missing unique SHA256 manifest.' }
$url = "https://forgejo.quantum-forge.io/riley/codetether-agent/releases/download/$tag/$name"
if ($assets[0].browser_download_url -cne $url) { throw 'UNEXPECTED_CHECKSUM_URL' }
$path = Join-Path $Work $name
Invoke-WebRequest $url -OutFile $path -UseBasicParsing
$entries = @(foreach ($line in Get-Content -LiteralPath $path) {
    if ($line -cmatch '^([0-9a-fA-F]{64}) [ *](.+)$' -and $Matches[2] -ceq $AssetName) {
        $Matches[1]
    }
})
if ($entries.Count -ne 1) { throw 'RELEASE_DIGEST_UNAVAILABLE: Missing or duplicate asset checksum.' }
$entries[0]