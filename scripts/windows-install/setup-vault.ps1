# Restore the original interactive Vault-backed provider configuration.
param([string]$CodetetherPath)
$address = if ($env:VAULT_ADDR) { $env:VAULT_ADDR } else { [Environment]::GetEnvironmentVariable('VAULT_ADDR', 'User') }
$existing = if ($env:VAULT_TOKEN) { $env:VAULT_TOKEN } else { [Environment]::GetEnvironmentVariable('VAULT_TOKEN', 'User') }
if ([string]::IsNullOrWhiteSpace($existing) -or $existing -eq 'hvs.your-token') { $existing = $null }
$model = if ($env:CODETETHER_DEFAULT_MODEL) { $env:CODETETHER_DEFAULT_MODEL } else { [Environment]::GetEnvironmentVariable('CODETETHER_DEFAULT_MODEL', 'User') }
if ($address -and $existing -and $model) { Write-Host 'Vault and model settings already configured; existing credentials preserved.'; return }
$settings = & "$PSScriptRoot\vault-settings.ps1" -Address $address -Existing $existing -Model $model
if ($null -eq $settings) { return }
$address = $settings.Address; $model = $settings.Model
$uri = $null
if (-not [Uri]::TryCreate($address, [UriKind]::Absolute, [ref]$uri) -or $uri.Scheme -notin @('https','http') -or $uri.UserInfo -or $uri.Query -or $uri.Fragment) {
    $settings.Token.Dispose()
    throw 'VAULT_ADDR_INVALID: Enter an HTTP(S) Vault address without embedded credentials.'
}
& "$PSScriptRoot\save-vault.ps1" -Address $address -Token $settings.Token
if (-not $model) {
    try {
        $catalog = & $CodetetherPath models --json 2>$null | ConvertFrom-Json
        $first = @($catalog)[0]
        if ($first.provider -and $first.id) { $model = "$($first.provider)/$($first.id)" }
    } catch { Write-Warning 'Automatic model discovery was unavailable; your token is saved.' }
    if (-not $model -and -not [Console]::IsInputRedirected) { $model = Read-Host 'CODETETHER_DEFAULT_MODEL (provider/model, or Enter to configure later)' }
}
if ($model) {
    [Environment]::SetEnvironmentVariable('CODETETHER_DEFAULT_MODEL', $model, 'User')
    $env:CODETETHER_DEFAULT_MODEL = $model
}