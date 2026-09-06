param([string]$Root, [string]$Work)
$global:vaultFixtureValue = 'fixture-' + [guid]::NewGuid()
$global:hiddenPrompt = $false
function Read-Host {
    param([string]$Prompt, [switch]$AsSecureString)
    $global:hiddenPrompt = $AsSecureString.IsPresent
    if (-not $global:vaultFixtureValue) { return New-Object Security.SecureString }
    ConvertTo-SecureString $global:vaultFixtureValue -AsPlainText -Force
}
$result = @(& "$Root/vault-token.ps1")
Assert-Contract ($global:hiddenPrompt) 'Vault token is requested with hidden input'
Assert-Contract ($result.Count -eq 1 -and $result[0] -is [Security.SecureString]) 'no plaintext token emitted'
$result[0].Dispose()
$global:vaultFixtureValue = ''
Assert-Throws { & "$Root/vault-token.ps1" } 'VAULT_TOKEN_REQUIRED'
$kept = & "$Root/vault-token.ps1" -Existing ('existing-fixture-' + [guid]::NewGuid())
Assert-Contract ($kept -is [Security.SecureString]) 'blank input preserves existing token securely'
$kept.Dispose()
$entry = Get-Content "$Root/entry.ps1" -Raw
Assert-Contract ($entry.Contains('setup-vault.ps1')) 'MSI and launcher installer restore original Vault prompt'
Assert-Contract ($entry.IndexOf('register.ps1') -lt $entry.IndexOf('setup-vault.ps1')) 'Vault setup follows verified packaged activation'
$settings = Get-Content "$Root/vault-settings.ps1" -Raw
Assert-Contract ($settings.Contains('vault-dialog.ps1')) 'no-console MSI path has an interactive credential dialog'
$dialog = Get-Content "$Root/vault-dialog.ps1" -Raw
Assert-Contract ($dialog.Contains('UseSystemPasswordChar = $true')) 'dialog masks the token'
$save = Get-Content "$Root/save-vault.ps1" -Raw
Assert-Contract ($save.Contains("SetEnvironmentVariable('VAULT_TOKEN', `$value, 'User')")) 'original user-scoped persistence is retained'
Assert-Contract ($save.Contains('ZeroFreeBSTR')) 'unmanaged secret buffer is cleared'
Assert-Contract ($save -notmatch '(Write-Host|Write-Output|Write-Error).*\$value') 'token is not sent to output/log APIs'
Assert-Contract ($entry -notmatch '--(token|vault-token)') 'credential is not passed in a process command line'
