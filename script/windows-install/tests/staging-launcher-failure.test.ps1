# Mocked local bootstrap failures must never unblock or execute a damaged helper.
param([string]$Root, [string]$Work)
$launcher = Get-Content -LiteralPath (Join-Path $Root 'Install-CodeTether.cmd') -Raw
$command = [regex]::Match($launcher, '-Command "(.*)"').Groups[1].Value
$guard = '$local -notmatch ''^[A-Za-z]:\\'' -or ([IO.DriveInfo]::new($local)).DriveType -ne ''Fixed'''
$command = $command.Replace($guard, '$false').Replace('exit $code', 'return $code')
$source = Join-Path $Work 'launcher-failure-source'
& "$PSScriptRoot/staging-fixture.ps1" -Root $Root -Directory $source
$savedLocal = $env:LOCALAPPDATA; $savedBundle = $env:CODETETHER_BUNDLE_DIR
$env:LOCALAPPDATA = Join-Path $Work 'failure-localappdata'; $env:CODETETHER_BUNDLE_DIR = $source
function Get-ExecutionPolicy { param([switch]$List) [pscustomobject]@{ Scope = 'Process'; ExecutionPolicy = 'RemoteSigned' } }
function Unblock-File { throw 'UNEXPECTED_UNBLOCK' }
try {
    $bootstrap = Join-Path $source 'script/windows-install/stage-bundle.ps1'
    Move-Item -LiteralPath $bootstrap -Destination "$bootstrap.retained"
    $log = Join-Path $Work 'missing-bootstrap.log'
    $result = & ([scriptblock]::Create($command)) 2> $log
    Assert-Contract ($result -eq 1) 'missing bootstrap rejected'
    Assert-Contract ((Get-Content -LiteralPath $log -Raw) -match 'BUNDLE_INCOMPLETE') 'typed missing bootstrap rejection'
    Move-Item -LiteralPath "$bootstrap.retained" -Destination $bootstrap
    function Copy-Item([string]$LiteralPath, [string]$Destination) { Set-Content -LiteralPath $Destination -Value 'corrupt bootstrap' }
    $log = Join-Path $Work 'corrupt-bootstrap.log'
    $result = & ([scriptblock]::Create($command)) 2> $log
    Assert-Contract ($result -eq 1) 'corrupt bootstrap rejected'
    Assert-Contract ((Get-Content -LiteralPath $log -Raw) -match 'BUNDLE_INTEGRITY_FAILED') 'typed bootstrap integrity rejection'
    $stages = @(Get-ChildItem -LiteralPath (Join-Path $env:LOCALAPPDATA 'codetether/install-stages') -Directory)
    Assert-Contract ($stages.Count -eq 2) 'separate failed stages retained'
    foreach ($stage in $stages) { Assert-Contract (Test-Path -LiteralPath (Join-Path $stage.FullName 'bootstrap.log')) 'failure transcript retained' }
} finally { $env:LOCALAPPDATA = $savedLocal; $env:CODETETHER_BUNDLE_DIR = $savedBundle }
