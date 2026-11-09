# Static/local parser and mocked-local contract checks; never invokes Windows servicing.
param([string]$EvidenceRoot)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$root = Split-Path $PSScriptRoot
if (-not $EvidenceRoot) { $EvidenceRoot = Join-Path (Split-Path (Split-Path $root)) 'artifacts/windows-installer-evidence' }
$work = Join-Path $EvidenceRoot "tests-$([guid]::NewGuid())"
New-Item -ItemType Directory $work -Force | Out-Null
Start-Transcript -Path (Join-Path $work 'contract-tests.log') | Out-Null
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
$files = @((Join-Path (Split-Path (Split-Path $root)) 'install.ps1')) + @(Get-ChildItem $root -Filter '*.ps1' -File | ForEach-Object FullName)
foreach ($file in $files) {
    $tokens = $null; $errors = $null
    [void][Management.Automation.Language.Parser]::ParseFile($file, [ref]$tokens, [ref]$errors)
    Assert-Contract ($errors.Count -eq 0) "$file syntax: $errors"
    $codeLines = @(Get-Content $file | Where-Object { $_.Trim() -and -not $_.TrimStart().StartsWith('#') })
    Assert-Contract ($codeLines.Count -le 50) "$file exceeds 50 nonblank/noncomment lines"
}
Write-Host "static/local: parsed $($files.Count) installer scripts; each <=50 code lines."
foreach ($test in Get-ChildItem $PSScriptRoot -Filter '*.test.ps1' | Sort-Object Name) {
    & $test.FullName -Root $root -Work $work
    Write-Host "mocked local: $($test.Name) assertions succeeded."
}
Write-Host "Evidence fixtures retained: $work"
Stop-Transcript | Out-Null