param([string]$Root, [string]$Work)
$global:probeFixture = @{ Json = ''; Exit = 0; Throw = $false }
$probeRoot = Join-Path $Work 'probe-fixture'
New-Item -ItemType Directory $probeRoot | Out-Null
Copy-Item "$Root/probe-ocr.ps1" $probeRoot
@'
param($Alias,$Work)
    [IO.File]::WriteAllText((Join-Path $Work 'ocr-status.json'), $global:probeFixture.Json)
    [IO.File]::WriteAllText((Join-Path $Work 'ocr-status.stderr.log'), 'fixture warning')
    if ($global:probeFixture.Throw) { throw 'fixture activation failure' }
    $global:probeFixture.Exit
'@ | Set-Content (Join-Path $probeRoot 'run-ocr-probe.ps1')
foreach ($case in (& "$PSScriptRoot/readiness-cases.ps1")) {
    $directory = Join-Path $Work "probe-$($case.Name)"
    New-Item -ItemType Directory $directory | Out-Null
    $global:probeFixture.Json = $case.Json; $global:probeFixture.Exit = $case.Exit
    $action = { & "$probeRoot/probe-ocr.ps1" -Alias 'fixture-alias' -Work $directory }
    if ($case.Error) { Assert-Throws $action $case.Error } else { & $action }
    Assert-Contract (Test-Path (Join-Path $directory 'ocr-status.json')) 'probe JSON evidence retained on every outcome'
    Assert-Contract (Test-Path (Join-Path $directory 'ocr-status.stderr.log')) 'probe stderr evidence retained on every outcome'
}
$global:probeFixture.Throw = $true
$directory = Join-Path $Work 'probe-launch-failure'
New-Item -ItemType Directory $directory | Out-Null
Assert-Throws { & "$probeRoot/probe-ocr.ps1" -Alias 'fixture-alias' -Work $directory } 'OCR_PROBE_FAILED'
$register = Get-Content "$Root/register.ps1" -Raw
Assert-Contract ($register.IndexOf('alias-tag.ps1') -lt $register.IndexOf('probe-ocr.ps1')) 'AppExecLink checked before native probe'
Assert-Contract ($register.IndexOf('probe-ocr.ps1') -lt $register.IndexOf('publish-alias.ps1')) 'native readiness required before PATH/legacy changes'
Assert-Contract ($register -match 'probe-ocr.ps1" -Alias \$alias') 'probe uses the activated packaged alias'