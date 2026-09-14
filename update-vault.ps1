# Update Vault credentials without reinstalling CodeTether or registering MSIX.
# Usage: irm https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/update-vault.ps1 | iex
& {
$ErrorActionPreference = 'Stop'
if ($env:OS -ne 'Windows_NT') { throw 'WINDOWS_REQUIRED: Run this command in a normal Windows PowerShell.' }
$blocked = @(Get-ExecutionPolicy -List | Where-Object { $_.ExecutionPolicy -eq 'AllSigned' -or ($_.Scope -in @('MachinePolicy','UserPolicy') -and $_.ExecutionPolicy -eq 'Restricted') })
if ($blocked.Count) { throw 'SIGNED_UPDATER_REQUIRED: Organization and signing policies are preserved; ask your administrator for an approved updater.' }
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($identity)
if ($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) { throw 'PER_USER_REQUIRED: Open PowerShell normally, not as administrator.' }
$commit = '30874824184bc783cb3e5980617cd8afaa4d23e7'
$base = 'https://raw.githubusercontent.com/rileyseaburg/codetether-agent'
$api = 'https://api.github.com/repos/rileyseaburg/codetether-agent'
$headers = @{ 'User-Agent' = 'codetether-vault-updater' }
if (-not $env:LOCALAPPDATA -or $env:LOCALAPPDATA -notmatch '^[A-Za-z]:\\' -or ([IO.DriveInfo]::new($env:LOCALAPPDATA)).DriveType -ne 'Fixed') { throw 'LOCAL_STAGE_REQUIRED: Updater helpers require a local fixed drive.' }
$listing = Invoke-RestMethod "$api/contents/script/windows-install?ref=$commit" -Headers $headers
$stage = Join-Path $env:LOCALAPPDATA "codetether\vault-recovery\$([guid]::NewGuid())"
New-Item -ItemType Directory $stage -Force | Out-Null
foreach ($name in @('repair-vault.ps1', 'verify-vault-token.ps1', 'save-vault.ps1', 'vault-token.ps1')) {
    $entry = @($listing | Where-Object { $_.type -eq 'file' -and $_.path -ceq "script/windows-install/$name" })
    if ($entry.Count -ne 1 -or $entry[0].sha -notmatch '^[0-9a-f]{40}$') { throw 'UPDATER_SOURCE_INVALID' }
    $file = Join-Path $stage $name
    Invoke-WebRequest "$base/$commit/script/windows-install/$name" -OutFile $file -UseBasicParsing
    $bytes = [IO.File]::ReadAllBytes($file)
    $prefix = [Text.Encoding]::UTF8.GetBytes("blob $($bytes.Length)`0")
    $sha = [Security.Cryptography.SHA1]::Create()
    try { $hash = ([BitConverter]::ToString($sha.ComputeHash($prefix + $bytes))).Replace('-', '').ToLowerInvariant() }
    finally { $sha.Dispose() }
    if ($hash -cne $entry[0].sha) { throw 'UPDATER_INTEGRITY_FAILED: Downloaded helper was not executed.' }
    Unblock-File -LiteralPath $file
}
Write-Host "Verified updater helpers retained at $stage"
$system = if ([Environment]::Is64BitProcess) { 'System32' } else { 'Sysnative' }
$powershell = Join-Path $env:SystemRoot "$system\WindowsPowerShell\v1.0\powershell.exe"
& $powershell -NoProfile -ExecutionPolicy RemoteSigned -File (Join-Path $stage 'repair-vault.ps1')
if ($LASTEXITCODE -ne 0) { throw 'VAULT_UPDATE_FAILED: The current terminal was not changed. See the updater diagnostic.' }
$address = [Environment]::GetEnvironmentVariable('VAULT_ADDR', 'User')
$token = [Environment]::GetEnvironmentVariable('VAULT_TOKEN', 'User')
if ([string]::IsNullOrWhiteSpace($address) -or [string]::IsNullOrWhiteSpace($token)) { throw 'VAULT_UPDATE_NOT_SAVED' }
$env:VAULT_ADDR = $address; $env:VAULT_TOKEN = $token
Remove-Variable token, address
Write-Host 'Verified Vault settings are saved for your Windows user and reloaded into THIS PowerShell.'
Write-Host 'Run codetether models here. Other already-running terminals and CodeTether processes retain their old environment.'
}