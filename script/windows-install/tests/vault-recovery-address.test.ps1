# Mocked-local security checks: reject unsafe addresses without sending a token.
param([string]$Root, [string]$Work)
$global:addressRequests = 0
function Invoke-RestMethod { param($Uri, $Headers, $Method, $TimeoutSec); $global:addressRequests++; throw 'unexpected request' }
$token = ConvertTo-SecureString ('fixture-' + [guid]::NewGuid()) -AsPlainText -Force
try {
    foreach ($address in @('http://vault.example.invalid', 'https://user:password@vault.example.invalid', 'https://vault.example.invalid?token=no', 'https://vault.example.invalid#fragment', 'not-a-url')) {
        Assert-Throws { & "$Root/verify-vault-token.ps1" -Address $address -Token $token } 'VAULT_ADDR_INVALID'
    }
    Assert-Contract ($global:addressRequests -eq 0) 'unsafe addresses rejected before HTTP calls'
} finally { $token.Dispose() }
$script = Get-Content "$Root/repair-vault.ps1" -Raw
$save = Get-Content "$Root/save-vault.ps1" -Raw
Assert-Contract ($script -notmatch 'Start-Transcript|Write-Host.*\$token|Write-Host.*\$value') 'token not logged by recovery'
Assert-Contract ($save.Contains("SetEnvironmentVariable('VAULT_TOKEN', `$value, 'User')")) 'replacement persisted for the current user'
Assert-Contract ($save.Contains('$env:VAULT_TOKEN = $value')) 'updater process receives the replacement'
Remove-Variable addressRequests -Scope Global
