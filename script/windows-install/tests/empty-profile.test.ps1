param([string]$Root, [string]$Work)
$env:SystemRoot = $Work
$global:emptyProfileCommand = ''
function Start-Process {
    param($FilePath, $Verb, $ArgumentList, [switch]$Wait, [switch]$PassThru)
    $global:emptyProfileCommand = [Text.Encoding]::Unicode.GetString([Convert]::FromBase64String($ArgumentList[-1]))
    [pscustomobject]@{ ExitCode = 0 }
}
& "$Root/elevate.ps1" -Languages @()
Assert-Contract ($global:emptyProfileCommand.Contains("'W10='")) 'empty profile encoded as JSON [], not [null]'
$entry = Get-Content "$Root/entry.ps1" -Raw
Assert-Contract ($entry -notmatch "throw 'NO_USER_LANGUAGES") 'empty profile reaches installed-capability selection and native probe'
