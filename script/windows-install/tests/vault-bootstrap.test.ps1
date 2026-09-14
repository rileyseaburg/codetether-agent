# Static/local bootstrap contracts; no Windows process or registry is invoked.
param([string]$Root, [string]$Work)
$file = Join-Path (Split-Path (Split-Path $Root)) 'update-vault.ps1'
$text = Get-Content $file -Raw
$tokens = $null; $errors = $null
[void][Management.Automation.Language.Parser]::ParseFile($file, [ref]$tokens, [ref]$errors)
Assert-Contract ($errors.Count -eq 0) 'Vault bootstrap parses'
$count = @(Get-Content $file | Where-Object { $_.Trim() -and -not $_.TrimStart().StartsWith('#') }).Count
Assert-Contract ($count -le 50) 'Vault bootstrap stays within its line budget'
Assert-Contract ($text -match "\`$commit = '[0-9a-f]{40}'") 'helper commit is immutable'
Assert-Contract ($text.Contains('/raw/commit/$commit/script/windows-install/$name')) 'only pinned helpers are fetched'
Assert-Contract ($text.IndexOf('UPDATER_INTEGRITY_FAILED') -lt $text.IndexOf('Unblock-File')) 'hash verified before trusting a file'
Assert-Contract ($text.IndexOf('Unblock-File') -lt $text.IndexOf('& $powershell')) 'helpers verified before execution'
Assert-Contract ($text.Contains('MachinePolicy') -and $text.Contains('UserPolicy') -and $text.Contains('AllSigned')) 'stronger policy is checked before child launch'
Assert-Contract ($text -notmatch 'ExecutionPolicy\s+(Bypass|Unrestricted)|Set-ExecutionPolicy|Start-Transcript') 'no global policy bypass or token transcript'
Assert-Contract ($text.Contains('LOCAL_STAGE_REQUIRED') -and $text.Contains('PER_USER_REQUIRED')) 'normal-user local staging required'
Assert-Contract ($text.IndexOf('$LASTEXITCODE -ne 0') -lt $text.IndexOf('$env:VAULT_TOKEN = $token')) 'caller environment changed only after successful update'
Assert-Contract ($text.Contains("GetEnvironmentVariable('VAULT_TOKEN', 'User')")) 'caller explicitly reloads the saved token'
Assert-Contract ($text -notmatch 'Add-AppxPackage|makeappx|register\.ps1|probe-ocr') 'token update independent of failed MSIX registration'
Assert-Contract ($text -notmatch '(?m)^& \$powershell.*\$(token|secret)') 'token never passed in process arguments'
