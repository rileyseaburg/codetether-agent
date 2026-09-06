param([string]$Root, [string]$Work)
$global:capabilityFixture = @{ Installed = $false; AddCount = 0; Reboot = $false; Offer = $true }
function Get-WindowsCapability {
    param([switch]$Online, [string]$Name, [string]$ErrorAction)
    if (-not $global:capabilityFixture.Offer) { return }
    [pscustomobject]@{ Name = 'Language.OCR~~~en-US~0.0.1.0'; State = $(if ($global:capabilityFixture.Installed) { 'Installed' } else { 'NotPresent' }) }
}
function Add-WindowsCapability {
    param([switch]$Online, [string]$Name, [string]$ErrorAction)
    $global:capabilityFixture.Installed = $true; $global:capabilityFixture.AddCount++
    [pscustomobject]@{ RestartNeeded = $global:capabilityFixture.Reboot }
}
function Encode-Languages([string]$Json) { [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($Json)) }
$data = Encode-Languages '["en-US"]'
& "$Root/ocr-capabilities.ps1" -LanguagesBase64 $data
Assert-Contract ($global:capabilityFixture.AddCount -eq 1) 'missing language installed'
& "$Root/ocr-capabilities.ps1" -LanguagesBase64 $data
Assert-Contract ($global:capabilityFixture.AddCount -eq 1) 'installed language not reinstalled'
$global:capabilityFixture.Offer = $false
Assert-Throws { & "$Root/ocr-capabilities.ps1" -LanguagesBase64 $data } 'OCR_CAPABILITY_CATALOG_EMPTY'
Assert-Throws { & "$Root/ocr-capabilities.ps1" -LanguagesBase64 (Encode-Languages '[]') } 'OCR_CAPABILITY_CATALOG_EMPTY'
$global:capabilityFixture.Offer = $true; $global:capabilityFixture.Installed = $false; $global:capabilityFixture.Reboot = $true
Assert-Throws { & "$Root/ocr-capabilities.ps1" -LanguagesBase64 $data } 'OCR_RESTART_REQUIRED'
Assert-Throws { & "$Root/ocr-capabilities.ps1" -LanguagesBase64 (Encode-Languages '["en-US;calc"]') } 'INVALID_LANGUAGE_TAG'