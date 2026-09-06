# Execute the real inline command with a mocked fixed-drive check and Windows cmdlets.
param([string]$Root, [string]$Work)
$launcher = Get-Content -LiteralPath (Join-Path $Root 'Install-CodeTether.cmd') -Raw
$command = [regex]::Match($launcher, '-Command "(.*)"').Groups[1].Value
$tokens = $null; $errors = $null
[void][Management.Automation.Language.Parser]::ParseInput($command, [ref]$tokens, [ref]$errors)
Assert-Contract ($errors.Count -eq 0) 'launcher PowerShell syntax'
Assert-Contract ($launcher -notmatch 'ExecutionPolicy\s+(Bypass|Unrestricted)|Set-ExecutionPolicy|pushd') 'no policy bypass or mapped-drive dependency'
Assert-Contract ($launcher -match 'DisableDelayedExpansion' -and $launcher -match 'exit /b %RESULT%') 'CMD path and failure preservation'
$guard = '$local -notmatch ''^[A-Za-z]:\\'' -or ([IO.DriveInfo]::new($local)).DriveType -ne ''Fixed'''
Assert-Contract ($command.Contains($guard)) 'production checks local fixed drive'
$command = $command.Replace($guard, '$false').Replace('exit $code', 'return $code')
$source = Join-Path $Work 'launcher-source'
& "$PSScriptRoot/staging-fixture.ps1" -Root $Root -Directory $source
$savedLocal = $env:LOCALAPPDATA; $savedBundle = $env:CODETETHER_BUNDLE_DIR
$env:LOCALAPPDATA = Join-Path $Work 'mock-localappdata'; $env:CODETETHER_BUNDLE_DIR = $source
$script:policy = @([pscustomobject]@{ Scope = 'LocalMachine'; ExecutionPolicy = 'Restricted' })
$script:unblocked = [Collections.Generic.List[string]]::new()
function Get-ExecutionPolicy { param([switch]$List) $script:policy }
function Unblock-File {
    param([Parameter(ValueFromPipelineByPropertyName)][Alias('FullName')][string]$LiteralPath)
    process {
        Assert-Contract ($LiteralPath.StartsWith($env:LOCALAPPDATA)) 'only local copies unblocked'
        $script:unblocked.Add($LiteralPath)
    }
}
$location = Get-Location
try {
    $result = & ([scriptblock]::Create($command))
    Assert-Contract ($result -eq 0) 'local bootstrap succeeded with mocked Windows cmdlets'
    $stages = @(Get-ChildItem -LiteralPath (Join-Path $env:LOCALAPPDATA 'codetether/install-stages') -Directory)
    Assert-Contract ($stages.Count -eq 1) 'unique local staging directory'
    Assert-Contract (Test-Path -LiteralPath (Join-Path $stages[0].FullName 'installer-called.txt')) 'local installer received local exe'
    $helperCount = @(Get-ChildItem -LiteralPath (Join-Path $source 'script/windows-install') -Filter '*.ps1' -File).Count
    Assert-Contract ($script:unblocked.Count -eq ($helperCount + 2)) 'bootstrap and all staged script copies unblocked'
    Assert-Contract (Test-Path -LiteralPath (Join-Path $stages[0].FullName 'bootstrap.log')) 'transcript retained'
    & "$PSScriptRoot/staging-policy-cases.ps1" -Command $command -Work $Work
} finally {
    Set-Location -LiteralPath $location.Path
    $env:LOCALAPPDATA = $savedLocal; $env:CODETETHER_BUNDLE_DIR = $savedBundle
}