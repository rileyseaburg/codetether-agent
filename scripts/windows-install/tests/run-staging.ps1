# Standalone static/local parser and mocked local staging checks; retains every fixture.
param([string]$EvidenceRoot)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$root = Split-Path $PSScriptRoot
if (-not $EvidenceRoot) { $EvidenceRoot = Join-Path $root '../../artifacts/unc-staging' }
$work = Join-Path $EvidenceRoot "tests-$([guid]::NewGuid())"
New-Item -ItemType Directory -Path $work -Force | Out-Null
Start-Transcript -Path (Join-Path $work 'staging-tests.log') | Out-Null
function Assert-Contract([bool]$Condition, [string]$Message) {
    if (-not $Condition) { throw "ASSERTION_FAILED: $Message" }
}
function Assert-Throws([scriptblock]$Action, [string]$Pattern) {
    try { & $Action } catch {
        Assert-Contract ($_.Exception.Message -match $Pattern) "Expected $Pattern; got $($_.Exception.Message)"
        return
    }
    throw "ASSERTION_FAILED: Expected failure $Pattern"
}
Write-Host "Evidence retained at $work"
$files = @((Join-Path $root 'stage-bundle.ps1'), $PSCommandPath)
$files += @(Get-ChildItem -LiteralPath $PSScriptRoot -Filter 'staging*.ps1' -File | ForEach-Object FullName)
foreach ($file in $files) {
    $tokens = $null; $errors = $null
    [void][Management.Automation.Language.Parser]::ParseFile($file, [ref]$tokens, [ref]$errors)
    Assert-Contract ($errors.Count -eq 0) "$file syntax"
    $lines = @(Get-Content -LiteralPath $file | Where-Object { $_.Trim() -and -not $_.TrimStart().StartsWith('#') })
    Assert-Contract ($lines.Count -le 50) "$file <=50 code lines"
}
Write-Host "static/local: parsed $($files.Count) scripts; all <=50 code lines"
foreach ($test in @('staging.test.ps1', 'staging-launcher.test.ps1', 'staging-launcher-failure.test.ps1')) {
    & (Join-Path $PSScriptRoot $test) -Root $root -Work $work
    Write-Host "mocked local: $test assertions succeeded"
}
Stop-Transcript | Out-Null
