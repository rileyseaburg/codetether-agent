# Hidden input only; never echo tokens or insert them into process arguments.
param([string]$Existing)
$prompt = if ($Existing) { 'VAULT_TOKEN (hidden; Enter keeps the configured token)' } else { 'VAULT_TOKEN (hidden)' }
$secure = Read-Host $prompt -AsSecureString
if ($secure.Length -eq 0) {
    $secure.Dispose()
    if ($Existing) { return ConvertTo-SecureString -String $Existing -AsPlainText -Force }
    throw 'VAULT_TOKEN_REQUIRED: Enter a token or decline configuration; no placeholder is stored.'
}
return $secure