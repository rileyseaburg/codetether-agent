# Windows servicing/Appx cmdlets require native Windows PowerShell, also when invoked from pwsh.
param([System.Collections.IDictionary]$Options)
$data = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes((ConvertTo-Json -InputObject $Options -Compress)))
$entry = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes((Join-Path $PSScriptRoot 'entry.ps1')))
$command = @"
`$ErrorActionPreference = 'Stop'
try {
    `$values = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('$data')) | ConvertFrom-Json
    `$options = @{}
    foreach (`$property in `$values.PSObject.Properties) {
        if (`$property.Name -in @('FunctionGemma', 'FunctionGemmaOnly', 'Force')) { `$options[`$property.Name] = [bool]`$property.Value.IsPresent }
        else { `$options[`$property.Name] = `$property.Value }
    }
    `$entry = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('$entry'))
    & `$entry @options
    exit 0
} catch { Write-Error `$_.Exception.Message -ErrorAction Continue; exit 1 }
"@
$encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($command))
$system = if ([Environment]::Is64BitProcess) { 'System32' } else { 'Sysnative' }
& "$env:SystemRoot\$system\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -EncodedCommand $encoded
if ($LASTEXITCODE -ne 0) { throw 'WINDOWS_POWERSHELL_SETUP_FAILED: See the preceding diagnostic.' }
$userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
$env:PATH = "$userPath;$env:PATH"
