# GitHub's release API supplies the expected digest; refuse unverifiable artifacts.
param([psobject]$Asset, [string]$Work)
$property = $Asset.PSObject.Properties['digest']
if (-not $property -or $property.Value -notmatch '^sha256:([0-9a-fA-F]{64})$') {
    throw 'RELEASE_DIGEST_UNAVAILABLE: Release asset has no GitHub SHA256 digest. Supply a trusted local -ExePath or signed -MsixPath.'
}
$expected = $Matches[1]
$uri = [uri]$Asset.browser_download_url
if ($uri.Scheme -ne 'https' -or $uri.Host -ne 'github.com' -or
    -not $uri.AbsolutePath.StartsWith('/rileyseaburg/codetether-agent/releases/download/')) { throw 'UNEXPECTED_RELEASE_URL' }
$path = Join-Path $Work $Asset.name
Invoke-WebRequest $uri.AbsoluteUri -OutFile $path -UseBasicParsing
if ((Get-FileHash $path -Algorithm SHA256).Hash -ine $expected) { throw 'RELEASE_INTEGRITY_FAILED' }
$path
