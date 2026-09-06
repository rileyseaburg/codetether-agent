param([string]$Root, [string]$Work)
$global:automaticOcr = @()
function Get-WindowsCapability {
    param([switch]$Online, [string]$Name, [string]$ErrorAction)
    foreach ($tag in @('fr-FR', 'en-US')) {
        $capability = "Language.OCR~~~$tag~0.0.1.0"
        if ($capability -like $Name) {
            [pscustomobject]@{ Name = $capability; State = $(if ($global:automaticOcr -contains $capability) { 'Installed' } else { 'NotPresent' }) }
        }
    }
}
function Add-WindowsCapability {
    param([switch]$Online, [string]$Name, [string]$ErrorAction)
    $global:automaticOcr += $Name
    [pscustomobject]@{ RestartNeeded = $false }
}
foreach ($json in @('[]', '["xx-YY"]')) {
    $global:automaticOcr = @()
    $data = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($json))
    & "$Root/ocr-capabilities.ps1" -LanguagesBase64 $data
    Assert-Contract ($global:automaticOcr.Count -eq 1) 'one fallback installed automatically'
    Assert-Contract ($global:automaticOcr[0] -eq 'Language.OCR~~~en-US~0.0.1.0') 'English fallback preferred over arbitrary catalog ordering'
}