# Verify Forgejo assets against the same release's SHA256 manifest.
param([psobject]$Asset, [psobject]$Release, [string]$Work)
if (-not $Release) { throw 'RELEASE_DIGEST_UNAVAILABLE: Release checksum manifest required.' }
$tag = $Release.tag_name
if ($tag -notmatch '^v?\d+\.\d+\.\d+[-.A-Za-z0-9]*$') { throw 'INVALID_RELEASE_TAG' }
$uri = [uri]$Asset.browser_download_url
if ($Asset.name -notmatch '^[A-Za-z0-9_.-]+$' -or $uri.UserInfo -or $uri.Query -or $uri.Fragment -or $uri.Port -ne 443 -or
    $uri.AbsoluteUri -cne "https://github.com/rileyseaburg/codetether-agent/releases/download/$tag/$($Asset.name)") { throw 'UNEXPECTED_RELEASE_URL' }
$expected = & "$PSScriptRoot\release-checksum.ps1" -AssetName $Asset.name -Release $Release -Work $Work
$path = Join-Path $Work $Asset.name
Invoke-WebRequest $uri.AbsoluteUri -OutFile $path -UseBasicParsing
if ((Get-FileHash $path -Algorithm SHA256).Hash -ine $expected) { throw 'RELEASE_INTEGRITY_FAILED' }
$path