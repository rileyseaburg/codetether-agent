# Mocked local bundle, real copy and SHA256 operations. No Windows installation.
param([string]$Root, [string]$Work)
$source = Join-Path $Work 'bundle [source] ! with spaces'
& "$PSScriptRoot/staging-fixture.ps1" -Root $Root -Directory $source
$destination = Join-Path $Work 'local-stage'
New-Item -ItemType Directory -Path $destination | Out-Null
& "$Root/stage-bundle.ps1" -Source $source -Destination $destination
$manifest = @(Get-Content -LiteralPath (Join-Path $destination 'bundle-hashes.json') -Raw | ConvertFrom-Json)
Assert-Contract ($manifest.Count -eq 34) 'exe, installer, 31 top-level helpers, adjacent DLL staged'
foreach ($entry in $manifest) {
    $from = [IO.File]::ReadAllBytes((Join-Path $source $entry.Path))
    $to = [IO.File]::ReadAllBytes((Join-Path $destination $entry.Path))
    Assert-Contract ([Convert]::ToBase64String($from) -ceq [Convert]::ToBase64String($to)) "exact bytes: $($entry.Path)"
    Assert-Contract ($entry.SourceSHA256 -ceq $entry.LocalSHA256) 'manifest hash equality'
}
Assert-Contract (-not (Test-Path -LiteralPath (Join-Path $destination 'README.md'))) 'unrelated files not staged'
Assert-Contract (-not (Test-Path -LiteralPath (Join-Path $destination 'script/windows-install/tests'))) 'nested tests not staged'
Assert-Contract (-not (Test-Path -LiteralPath (Join-Path $destination 'installer-called.txt'))) 'staging never executes installer'
$incomplete = Join-Path $Work 'incomplete-bundle'
& "$PSScriptRoot/staging-fixture.ps1" -Root $Root -Directory $incomplete
foreach ($relative in @('install.ps1', 'codetether.exe', 'script/windows-install/entry.ps1', 'script/windows-install/register.ps1')) {
    $file = Join-Path $incomplete $relative
    Move-Item -LiteralPath $file -Destination "$file.retained"
    Assert-Throws { & "$Root/stage-bundle.ps1" -Source $incomplete -Destination $destination } 'BUNDLE_INCOMPLETE'
    Move-Item -LiteralPath "$file.retained" -Destination $file
}
$corrupt = Join-Path $Work 'corrupt-local-stage'
New-Item -ItemType Directory -Path $corrupt | Out-Null
Set-Content -LiteralPath (Join-Path $corrupt 'codetether.exe') -Value 'wrong bytes'
Assert-Throws { & "$Root/stage-bundle.ps1" -Source $source -Destination $corrupt } 'BUNDLE_INTEGRITY_FAILED: codetether.exe'
$failed = @(Get-Content -LiteralPath (Join-Path $corrupt 'bundle-hashes.json') -Raw | ConvertFrom-Json)
Assert-Contract ($failed[-1].SourceSHA256 -ne $failed[-1].LocalSHA256) 'mismatch evidence retained'
$damaged = Join-Path $Work 'copy-corruption-stage'
function Copy-Item([string]$LiteralPath, [string]$Destination) { Set-Content -LiteralPath $Destination -Value 'simulated truncated network copy' }
Assert-Throws { & "$Root/stage-bundle.ps1" -Source $source -Destination $damaged } 'BUNDLE_INTEGRITY_FAILED: install.ps1'
Assert-Contract (Test-Path -LiteralPath (Join-Path $damaged 'bundle-hashes.json')) 'failed network copy evidence retained'