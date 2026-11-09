# Collect settings without exposing a token; MSI receives a dialog if no console exists.
param([string]$Address, [string]$Existing, [string]$Model)
if (-not [Environment]::UserInteractive) {
    Write-Warning 'Vault configuration requires an interactive user. Existing credentials were left unchanged.'; return $null
}
if ([Console]::IsInputRedirected) {
    $settings = & "$PSScriptRoot\vault-dialog.ps1" -Address $Address -Model $Model -HasToken ([bool]$Existing)
    if ($null -eq $settings) { return $null }
    if ($settings.Token.Length -eq 0 -and $Existing) {
        $settings.Token.Dispose()
        $settings.Token = ConvertTo-SecureString $Existing -AsPlainText -Force
    }
    return $settings
}
Write-Host 'CodeTether Vault setup: settings are saved in your Windows user environment, as in the original installer.'
if ((Read-Host 'Configure Vault-backed providers now? [Y/n]') -match '^(n|no)$') { return $null }
$entered = Read-Host 'VAULT_ADDR (Enter keeps the configured address)'
if ($entered) { $Address = $entered.Trim() }
$token = & "$PSScriptRoot\vault-token.ps1" -Existing $Existing
@{ Address = $Address; Token = $token; Model = $Model }
