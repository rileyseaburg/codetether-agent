param([string]$Root, [string]$Work)
$env:SystemRoot = $Work
$global:elevationFixture = @{ Denied = $false; ExitCode = 0; Captured = '' }
function Start-Process {
    param($FilePath, $Verb, $ArgumentList, [switch]$Wait, [switch]$PassThru)
    Assert-Contract ($Verb -eq 'RunAs') 'explicit UAC required'
    Assert-Contract ($ArgumentList -notcontains '-ExecutionPolicy') 'execution policy unchanged'
    $global:elevationFixture.Captured = [Text.Encoding]::Unicode.GetString([Convert]::FromBase64String($ArgumentList[-1]))
    if ($global:elevationFixture.Denied) { throw 'The operation was canceled by the user.' }
    [pscustomobject]@{ ExitCode = $global:elevationFixture.ExitCode }
}
& "$Root/elevate.ps1" -Languages @('en-US')
Assert-Contract ($global:elevationFixture.Captured -match 'TrustedPeople') 'only leaf trust operation embedded'
Assert-Contract ($global:elevationFixture.Captured -notmatch 'Get-Content|Invoke-WebRequest|Start-Process|signtool|makeappx') 'elevated command contains no external script/tool execution'
$tokens = $null; $errors = $null
[void][Management.Automation.Language.Parser]::ParseInput($global:elevationFixture.Captured, [ref]$tokens, [ref]$errors)
Assert-Contract ($errors.Count -eq 0) 'encoded elevated command parses'
$global:elevationFixture.Captured | Set-Content (Join-Path $Work 'elevated-command-fixture.ps1')
$global:elevationFixture.Denied = $true
Assert-Throws { & "$Root/elevate.ps1" -Languages @('en-US') } 'UAC_DENIED_OR_UNAVAILABLE'
$global:elevationFixture.Denied = $false; $global:elevationFixture.ExitCode = 1
Assert-Throws { & "$Root/elevate.ps1" -Languages @('en-US') } 'PREREQUISITE_FAILED'
Assert-Throws { & "$Root/elevate.ps1" -Certificate "';calc" -Languages @('en-US') } 'INVALID_CERTIFICATE_ENCODING'
