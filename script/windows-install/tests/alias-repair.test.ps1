param([string]$Root, [string]$Work)
$previousLocalAppData = $env:LOCALAPPDATA
function Add-AppxPackage {
    param([string]$Register, [switch]$DisableDevelopmentMode, [string]$ErrorAction)
    Assert-Contract $DisableDevelopmentMode.IsPresent 'repair registers existing signed package, not a developer package'
    Assert-Contract ($Register -eq $global:aliasRepairFixture.Manifest) 'repair uses registered package manifest'
    $global:aliasRepairFixture.Calls++
    if ($global:aliasRepairFixture.Mode -eq 'failed') { throw 'fixture package registration rejection' }
    if ($global:aliasRepairFixture.Mode -eq 'repaired') { [IO.File]::WriteAllText($global:aliasRepairFixture.Alias, 'alias fixture') }
}
try {
    foreach ($mode in @('existing', 'repaired', 'unavailable', 'failed')) {
        $directory = Join-Path $Work "alias-$mode"
        $env:LOCALAPPDATA = $directory
        $install = Join-Path $directory 'installed-package'
        $alias = Join-Path $directory 'Microsoft/WindowsApps/fixture/codetether.exe'
        New-Item -ItemType Directory $install, (Split-Path $alias) -Force | Out-Null
        $manifest = Join-Path $install 'AppxManifest.xml'
        [IO.File]::WriteAllText($manifest, '<Package />')
        $global:aliasRepairFixture = @{ Mode = $mode; Calls = 0; Manifest = $manifest; Alias = $alias }
        if ($mode -eq 'existing') { [IO.File]::WriteAllText($alias, 'alias fixture') }
        $registered = [pscustomobject]@{ PackageFamilyName = 'fixture'; InstallLocation = $install }
        $action = { & "$Root/resolve-alias.ps1" -Registered $registered -Work $directory }
        switch ($mode) {
            'failed' { Assert-Throws $action 'PACKAGE_ALIAS_REPAIR_FAILED' }
            'unavailable' { Assert-Throws $action 'PACKAGE_ALIAS_UNAVAILABLE' }
            default { $result = & $action; Assert-Contract ($result.Path -eq $alias) 'resolved family-scoped alias' }
        }
        $expected = if ($mode -eq 'existing') { 0 } else { 1 }
        Assert-Contract ($global:aliasRepairFixture.Calls -eq $expected) 'absent alias gets exactly one supported repair attempt'
        if ($expected) { Assert-Contract (Test-Path (Join-Path $directory 'alias-repair.log')) 'repair evidence retained' }
    }
} finally { $env:LOCALAPPDATA = $previousLocalAppData }
