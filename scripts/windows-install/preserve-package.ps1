# Retain previous packaged executable bytes before Windows performs an MSIX upgrade.
param([hashtable]$Package, [string]$Work)
foreach ($previous in @(Get-AppxPackage -Name $Package.Name)) {
    $source = Join-Path $previous.InstallLocation 'codetether.exe'
    if (-not (Test-Path -LiteralPath $source)) { throw 'PREVIOUS_PACKAGE_EXE_MISSING: Refusing upgrade without preserving the previous executable.' }
    $destination = Join-Path $Work "previous-$($previous.PackageFullName)"
    New-Item -ItemType Directory $destination | Out-Null
    Copy-Item -LiteralPath $source -Destination (Join-Path $destination 'codetether.exe')
    Get-ChildItem -LiteralPath $previous.InstallLocation -Filter '*.dll' -File | Copy-Item -Destination $destination
    $previous | Format-List Name, PackageFullName, PackageFamilyName, Version | Out-String | Set-Content (Join-Path $destination 'package.txt')
    if ((Get-FileHash $source -Algorithm SHA256).Hash -ne (Get-FileHash (Join-Path $destination 'codetether.exe') -Algorithm SHA256).Hash) {
        throw 'PREVIOUS_PACKAGE_BACKUP_MISMATCH'
    }
}
