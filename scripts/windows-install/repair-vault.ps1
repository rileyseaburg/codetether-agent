# Explicit local reauthentication, including when stale settings are already present.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
function User-Setting([string]$Name, [string]$Default) {
    $value = [Environment]::GetEnvironmentVariable($Name, 'User')
    if (-not $value) { $value = [Environment]::GetEnvironmentVariable($Name, 'Process') }
    if ($value) { return $value }; return $Default
}
$address = User-Setting 'VAULT_ADDR' ''
$mount = User-Setting 'VAULT_MOUNT' 'secret'
$path = User-Setting 'VAULT_SECRETS_PATH' 'codetether/providers'
Write-Host 'Replace the rejected Vault token. Enter the new token privately here, not in chat.'
Write-Host 'Credentials are saved for your Windows user only after Vault verifies access.'
$entered = Read-Host 'Vault address (Enter keeps the configured address)'
if ($entered) { $address = $entered.Trim() }
$uri = $null
if (-not [Uri]::TryCreate($address, [UriKind]::Absolute, [ref]$uri) -or $uri.UserInfo -or $uri.Query -or $uri.Fragment -or
    ($uri.Scheme -ne 'https' -and -not ($uri.Scheme -eq 'http' -and $uri.IsLoopback))) {
    throw 'VAULT_ADDR_INVALID: Use HTTPS (HTTP is allowed only for loopback development).'
}
Write-Host "Vault server: $($uri.GetLeftPart([UriPartial]::Authority))"
$token = & "$PSScriptRoot\vault-token.ps1"
try {
    $result = & "$PSScriptRoot\verify-vault-token.ps1" -Address $address -Token $token -Mount $mount -ProviderPath $path
    if (-not $result.Valid) { throw 'VAULT_VALIDATION_FAILED' }
    & "$PSScriptRoot\save-vault.ps1" -Address $address -Token $token
    $token = $null
    Write-Host "Vault access confirmed; $($result.ProviderCount) provider entries are visible."
    Write-Host 'Saved for your Windows user and this updater process. Other running processes keep their existing environment; the PowerShell bootstrap reloads its caller explicitly.'
} finally {
    if ($null -ne $token) { $token.Dispose() }
}