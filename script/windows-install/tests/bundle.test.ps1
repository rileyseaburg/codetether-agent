param([string]$Root, [string]$Work)
$bootstrap = Join-Path (Split-Path (Split-Path $Root)) 'install.ps1'
$oldOs = $env:OS
$env:OS = 'Windows_NT'
$global:bundleFixture = @{}
function Invoke-RestMethod { throw 'BUNDLED_SETUP_MUST_NOT_DISCOVER_NETWORK_RELEASES' }
try {
    foreach ($layout in @('script/windows-install', 'windows-install')) {
        $directory = Join-Path $Work ('bundle-' + $layout.Replace('/', '-'))
        $helpers = Join-Path $directory $layout
        New-Item -ItemType Directory $helpers -Force | Out-Null
        Copy-Item $bootstrap (Join-Path $directory 'install.ps1')
        Copy-Item (Join-Path $Root 'bundled-source.ps1') $helpers
        'param($ExePath,$MsixPath,$Version,$FunctionGemma,$FunctionGemmaOnly,$Force); $global:bundleFixture = @{ Exe = $ExePath; Msix = $MsixPath }' | Set-Content (Join-Path $helpers 'entry.ps1')
        $exe = Join-Path $directory 'codetether.exe'
        [IO.File]::WriteAllText($exe, 'bundled executable fixture; never executed')
        & (Join-Path $directory 'install.ps1')
        Assert-Contract ($global:bundleFixture.Exe -eq $exe) 'standard bundled installer automatically selects adjacent executable'
        $msix = Join-Path $directory 'codetether.msix'
        [IO.File]::WriteAllText($msix, 'signed-package path fixture; never registered')
        & (Join-Path $directory 'install.ps1')
        Assert-Contract ($global:bundleFixture.Msix -eq $msix -and -not $global:bundleFixture.Exe) 'bundled MSIX takes precedence'
        & (Join-Path $directory 'install.ps1') -ExePath $exe
        Assert-Contract ($global:bundleFixture.Exe -eq $exe -and -not $global:bundleFixture.Msix) 'explicit executable overrides bundle defaults'
    }
} finally { $env:OS = $oldOs }
