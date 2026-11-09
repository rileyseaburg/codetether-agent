# Mocked-local orchestration: a rejected candidate never reaches persistence.
param([string]$Root, [string]$Work)
$fixture = Join-Path $Work ('recovery-order-' + [guid]::NewGuid())
New-Item -ItemType Directory $fixture | Out-Null
Copy-Item "$Root/repair-vault.ps1" (Join-Path $fixture 'repair-vault.ps1')
Set-Content (Join-Path $fixture 'vault-token.ps1') "ConvertTo-SecureString 'fixture' -AsPlainText -Force"
Set-Content (Join-Path $fixture 'verify-vault-token.ps1') "throw 'VAULT_VALIDATION_FAILED'"
Set-Content (Join-Path $fixture 'save-vault.ps1') "Add-Content (Join-Path `$PSScriptRoot 'saved.marker') 'called'"
function Read-Host { param($Prompt); 'https://vault.example.invalid' }
Assert-Throws { & (Join-Path $fixture 'repair-vault.ps1') } 'VAULT_VALIDATION_FAILED'
Assert-Contract (-not (Test-Path (Join-Path $fixture 'saved.marker'))) 'no persistence after validation failure'
Set-Content (Join-Path $fixture 'verify-vault-token.ps1') '[pscustomobject]@{ Valid = $true; ProviderCount = 1 }'
& (Join-Path $fixture 'repair-vault.ps1')
Assert-Contract (Test-Path (Join-Path $fixture 'saved.marker')) 'verified candidate reaches persistence'
$before = @(Get-Content (Join-Path $fixture 'saved.marker')).Count
function Read-Host { param($Prompt); 'http://vault.example.invalid' }
Assert-Throws { & (Join-Path $fixture 'repair-vault.ps1') } 'VAULT_ADDR_INVALID'
Assert-Contract (@(Get-Content (Join-Path $fixture 'saved.marker')).Count -eq $before) 'insecure remote HTTP cannot reach persistence'
