# Register for the invoking user. Windows enforces package signatures and trust.
param([hashtable]$Package, [string]$Work)
try { Add-AppxPackage -Path $Package.Path -ErrorAction Stop }
catch { throw "PACKAGE_REGISTRATION_FAILED: $($_.Exception.Message) Existing executable was preserved. Check AppXDeploymentServer/Operational in Event Viewer." }
$registered = @(Get-AppxPackage -Name $Package.Name | Where-Object {
    $_.Publisher -eq $Package.Publisher -and $_.Version -eq [version]$Package.Version
})
if ($registered.Count -ne 1) { throw 'PACKAGE_REGISTRATION_NOT_CONFIRMED: Expected exactly one matching package.' }
$registered[0] | Format-List Name, PackageFullName, PackageFamilyName, Version, Status | Out-String | Set-Content (Join-Path $Work 'registration.txt')
$resolved = & "$PSScriptRoot\resolve-alias.ps1" -Registered $registered[0] -Work $Work
$aliasDir = $resolved.Directory; $alias = $resolved.Path
& "$PSScriptRoot\alias-tag.ps1" -Path $alias
try { & $alias --version *> (Join-Path $Work 'activation.log') }
catch { throw "PACKAGE_ACTIVATION_FAILED: $($_.Exception.Message)" }
if ($LASTEXITCODE -ne 0) { throw "PACKAGE_ACTIVATION_FAILED: Exit $LASTEXITCODE. See $Work\activation.log" }
& "$PSScriptRoot\probe-ocr.ps1" -Alias $alias -Work $Work
& "$PSScriptRoot\publish-alias.ps1" -AliasDir $aliasDir -Work $Work
