param([string]$Root, [string]$Work)
$global:recognizerFixture = @(
    [pscustomobject]@{ Name = 'Language.OCR~~~en-US~0.0.1.0'; State = 'NotPresent' },
    [pscustomobject]@{ Name = 'Language.OCR~~~fr-FR~0.0.1.0'; State = 'Installed' }
)
$global:recognizerQueries = @()
function Get-WindowsCapability {
    param([switch]$Online, [string]$Name, [string]$ErrorAction)
    $global:recognizerQueries += $Name
    $global:recognizerFixture | Where-Object { $_.Name -like $Name }
}
function Add-WindowsCapability { throw 'INSTALLED_RECOGNIZER_MUST_NOT_REQUIRE_NETWORK_SERVICING' }
foreach ($json in @('[]', '["xx-YY"]', '["en-US"]', '["fr-FR"]')) {
    $global:recognizerQueries = @()
    $data = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($json))
    & "$Root/ocr-capabilities.ps1" -LanguagesBase64 $data
    Assert-Contract ($global:recognizerQueries[-1] -eq 'Language.OCR~~~fr-FR~0.0.1.0') "installed recognizer selected for $json"
}
$global:recognizerFixture += [pscustomobject]@{ Name = 'Language.OCR~~~it-IT~0.0.1.0'; State = 'Installed' }
$data = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes('["it-IT"]'))
& "$Root/ocr-capabilities.ps1" -LanguagesBase64 $data
Assert-Contract ($global:recognizerQueries[-1] -eq 'Language.OCR~~~it-IT~0.0.1.0') 'installed profile recognizer takes precedence over another installed language'
