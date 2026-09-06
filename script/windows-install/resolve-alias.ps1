# Resolve the package-family alias, repairing its signed per-user registration once if absent.
param([psobject]$Registered, [string]$Work)
$directory = Join-Path $env:LOCALAPPDATA "Microsoft\WindowsApps\$($Registered.PackageFamilyName)"
$path = Join-Path $directory 'codetether.exe'
if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
    $manifest = Join-Path $Registered.InstallLocation 'AppxManifest.xml'
    $log = Join-Path $Work 'alias-repair.log'
    Write-Host 'Package alias is absent; refreshing the existing signed package registration for this user.'
    try {
        # This switch describes the registered package, not a change to global Developer Mode.
        Add-AppxPackage -Register $manifest -DisableDevelopmentMode -ErrorAction Stop *> $log
    } catch {
        throw "PACKAGE_ALIAS_REPAIR_FAILED: $($_.Exception.Message) See $log. No PATH or legacy executable change was made."
    }
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "PACKAGE_ALIAS_UNAVAILABLE: Signed package re-registration did not restore the AppExecLink. Setup stopped without readiness success; existing executable preserved. See $log."
    }
}
@{ Directory = $directory; Path = $path }
