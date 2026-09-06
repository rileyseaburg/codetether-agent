# MSI has already staged these files locally; do not execute setup from its source share.
$ErrorActionPreference = 'Stop'
$logging = $false
$result = 0
try {
    $logDirectory = Join-Path $env:LOCALAPPDATA "codetether\install-evidence\msi-$([guid]::NewGuid())"
    New-Item -ItemType Directory -Path $logDirectory -Force | Out-Null
    Start-Transcript -Path (Join-Path $logDirectory 'msi-entry.log') | Out-Null
    $logging = $true
    $blocked = @(Get-ExecutionPolicy -List | Where-Object {
        $_.ExecutionPolicy -eq 'AllSigned' -or
        ($_.Scope -in @('MachinePolicy','UserPolicy') -and $_.ExecutionPolicy -eq 'Restricted')
    })
    if ($blocked.Count -or $env:PSExecutionPolicyPreference -eq 'AllSigned') {
        throw 'SIGNED_INSTALLER_REQUIRED: Configured signing policy is preserved.'
    }
    $root = Split-Path (Split-Path $PSScriptRoot)
    if ($root -notmatch '^[A-Za-z]:\\' -or ([IO.DriveInfo]::new($root)).DriveType -ne 'Fixed') {
        throw 'MSI_LOCAL_STAGE_REQUIRED: Installer payload must be on a local fixed drive.'
    }
    $installer = Join-Path $root 'install.ps1'
    Write-Host 'MSI payload is installed; beginning native package and Vault setup.'
    Unblock-File -LiteralPath $installer
    Get-ChildItem -LiteralPath $PSScriptRoot -Filter '*.ps1' -File | Unblock-File
    & $installer -ExePath (Join-Path $root 'codetether.exe')
} catch {
    Write-Error $_.Exception.Message -ErrorAction Continue
    $result = 1
} finally {
    if ($logging) { Stop-Transcript | Out-Null }
}
exit $result