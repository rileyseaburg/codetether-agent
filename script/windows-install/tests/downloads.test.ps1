param([string]$Root, [string]$Work)
$global:downloadFixture = @{ Fail = $true; Expanded = $false }
function Invoke-WebRequest {
    param($Uri, $OutFile, [switch]$UseBasicParsing)
    if ($global:downloadFixture.Fail) { throw 'fixture network unavailable' }
    [IO.File]::WriteAllText($OutFile, 'not the expected archive')
}
function Expand-Archive { param($LiteralPath, $DestinationPath); $global:downloadFixture.Expanded = $true }
Assert-Throws { & "$Root/sdk.ps1" -Work $Work } 'SDK_UNAVAILABLE'
$global:downloadFixture.Fail = $false
Assert-Throws { & "$Root/sdk.ps1" -Work $Work } 'SDK_INTEGRITY_FAILED'
Assert-Contract (-not $global:downloadFixture.Expanded) 'corrupted SDK never extracted'
$asset = [pscustomobject]@{ name = 'codetether.exe'; browser_download_url = 'https://github.com/rileyseaburg/codetether-agent/releases/download/v1.0.0/codetether.exe' }
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Work $Work } 'RELEASE_DIGEST_UNAVAILABLE'
$asset | Add-Member -NotePropertyName digest -NotePropertyValue ('sha256:' + ('0' * 64))
Assert-Throws { & "$Root/download-asset.ps1" -Asset $asset -Work $Work } 'RELEASE_INTEGRITY_FAILED'
$bootstrap = Get-Content (Join-Path (Split-Path (Split-Path $Root)) 'install.ps1') -Raw
Assert-Contract ($bootstrap -match 'HELPER_INTEGRITY_FAILED' -and $bootstrap -match 'raw.githubusercontent.com/\$repo/\$commit/') 'helper download is commit-pinned and hash-checked'
$pin = [regex]::Match((Get-Content "$Root/sdk.ps1" -Raw), "sha512 = '([0-9a-f]{128})'").Groups[1].Value
$archive = Join-Path (Split-Path $Work) 'sdk.nupkg'
if (Test-Path $archive) {
    Assert-Contract ((Get-FileHash $archive -Algorithm SHA512).Hash -ieq $pin) 'downloaded official SDK bytes match pinned SHA512'
    Get-FileHash $archive -Algorithm SHA512 | Format-List | Out-String | Set-Content (Join-Path $Work 'sdk-sha512.txt')
}