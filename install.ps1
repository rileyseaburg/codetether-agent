# Windows package-identity installer. Supports irm .../install.ps1 | iex.
# Local use: .\install.ps1 [-ExePath .\codetether.exe]; adjacent bundled inputs are autodetected.
param([string]$ExePath, [string]$MsixPath, [string]$Version,
    [switch]$FunctionGemma, [switch]$FunctionGemmaOnly, [switch]$Force, [switch]$Help)
$ErrorActionPreference = 'Stop'
if ($Help) {
    Write-Host 'Usage: install.ps1 [-ExePath local.exe | -MsixPath signed.msix] [-Version release-tag] [-FunctionGemma[Only]] [-Force]'
    Write-Host 'Installs a per-user MSIX; explicit UAC installs OCR languages and optionally trusts a local signing leaf.'
    return
}
if ($env:OS -ne 'Windows_NT') { throw 'WINDOWS_REQUIRED: Run this installer on Windows.' }
$repo = 'rileyseaburg/codetether-agent'
$api = "https://api.github.com/repos/$repo"
$headers = @{ 'User-Agent' = 'codetether-installer' }
$helperRoot = if ($PSScriptRoot) { Join-Path $PSScriptRoot 'script\windows-install' } else { '' }
if ($PSScriptRoot -and -not (Test-Path (Join-Path $helperRoot 'entry.ps1'))) { $helperRoot = Join-Path $PSScriptRoot 'windows-install' }
if (-not $helperRoot -or -not (Test-Path (Join-Path $helperRoot 'entry.ps1'))) {
    # Reviewed helper revision, independent of binary tags. Never execute floating main helpers.
    $commit = 'e0c1a0485a29bed12fe025118a886fb3425ecb01'
    if ($commit -notmatch '^[0-9a-f]{40}$') { throw 'INVALID_RELEASE_COMMIT' }
    # List only this directory; recursive tree APIs can truncate large repositories.
    $listing = Invoke-RestMethod "$api/contents/script/windows-install?ref=$commit" -Headers $headers
    $helperRoot = Join-Path $env:LOCALAPPDATA "codetether\install-evidence\helpers-$commit-$([guid]::NewGuid())"
    New-Item -ItemType Directory $helperRoot -Force | Out-Null
    $files = @($listing | Where-Object { $_.type -eq 'file' -and $_.path -cmatch '^script/windows-install/[a-z0-9-]+\.ps1$' })
    if (-not $files.Count) { throw 'RELEASE_HAS_NO_WINDOWS_HELPERS: Use a bundled installer or checkout with -ExePath.' }
    foreach ($file in $files) {
        $path = Join-Path $helperRoot ([IO.Path]::GetFileName($file.path))
        Invoke-WebRequest "https://raw.githubusercontent.com/$repo/$commit/$($file.path)" -OutFile $path -UseBasicParsing
        $bytes = [IO.File]::ReadAllBytes($path)
        $prefix = [Text.Encoding]::UTF8.GetBytes("blob $($bytes.Length)`0")
        $hasher = [Security.Cryptography.SHA1]::Create()
        try { $hash = ([BitConverter]::ToString($hasher.ComputeHash($prefix + $bytes))).Replace('-', '').ToLowerInvariant() }
        finally { $hasher.Dispose() }
        if ($hash -cne $file.sha) { throw "HELPER_INTEGRITY_FAILED: $path" }
    }
    Write-Host "Installer helpers pinned to $commit; retained at $helperRoot"
}
if ($PSScriptRoot -and -not $ExePath -and -not $MsixPath -and -not $Version) {
    $bundle = & (Join-Path $helperRoot 'bundled-source.ps1') -Directory $PSScriptRoot
    $ExePath = $bundle.Exe; $MsixPath = $bundle.Msix
}
& (Join-Path $helperRoot 'entry.ps1') -ExePath $ExePath -MsixPath $MsixPath -Version $Version -FunctionGemma:$FunctionGemma -FunctionGemmaOnly:$FunctionGemmaOnly -Force:$Force