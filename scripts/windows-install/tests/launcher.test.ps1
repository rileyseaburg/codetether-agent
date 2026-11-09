param([string]$Root, [string]$Work)
$launcher = Get-Content -LiteralPath (Join-Path $Root 'Install-CodeTether.cmd') -Raw
$line = @($launcher -split "`n" | Where-Object { $_ -match 'powershell.exe' })[0]
$command = [regex]::Match($line, '-Command "(.*)"\s*$').Groups[1].Value
$tokens = $null; $errors = $null
[void][Management.Automation.Language.Parser]::ParseInput($command, [ref]$tokens, [ref]$errors)
Assert-Contract ($errors.Count -eq 0) 'one-click launcher PowerShell syntax'
Assert-Contract ($launcher -match 'CODETETHER_BUNDLE_DIR' -and $launcher -match 'DisableDelayedExpansion') 'bundle path is carried as data, not injected into PowerShell code'
Assert-Contract ($launcher -match 'ExecutionPolicy RemoteSigned') 'policy is process-local RemoteSigned'
Assert-Contract ($launcher -match 'AllSigned' -and $launcher -match 'MachinePolicy' -and $launcher -match 'SIGNED_INSTALLER_REQUIRED') 'stronger signing and organization policies stop bootstrap'
Assert-Contract ($launcher -notmatch 'ExecutionPolicy\s+(Bypass|Unrestricted)|Set-ExecutionPolicy') 'no global policy or bypass'
Assert-Contract ($launcher -match 'Unblock-File -LiteralPath \$installer') 'only trusted bundled installer files are explicitly unblocked'
Assert-Contract ($launcher -match 'exit /b %RESULT%') 'installer failure propagates'
