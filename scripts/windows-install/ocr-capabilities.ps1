# Elevated servicing uses the installer's profile, or an already installed recognizer.
param([string]$LanguagesBase64)
$languages = @([Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($LanguagesBase64)) | ConvertFrom-Json)
$available = @(Get-WindowsCapability -Online -Name 'Language.OCR*' -ErrorAction Stop)
$selected = @()
foreach ($language in $languages) {
    if ($language -notmatch '^[A-Za-z]{2,3}(-[A-Za-z0-9]{2,8})*$') { throw 'INVALID_LANGUAGE_TAG' }
    $matches = @($available | Where-Object { $_.Name -like "Language.OCR~~~$language~*" })
    # Windows OCR offers one capability for some regional language families (e.g. English).
    if (-not $matches.Count -and $language -notmatch '^zh(-|$)') {
        $base = ($language -split '-')[0]
        $matches = @($available | Where-Object { $_.Name -like "Language.OCR~~~$base-*" } | Sort-Object Name | Select-Object -First 1)
    }
    if ($matches.Count) { $selected += $matches[0].Name }
    else { Write-Warning "No Windows OCR capability matches current-user language $language." }
}
$installed = @($available | Where-Object { $_.State -eq 'Installed' } | Sort-Object Name)
$preferred = @($installed | Where-Object { $selected -contains $_.Name })
if ($preferred.Count) { $selected = @($preferred | ForEach-Object { $_.Name }) }
elseif ($installed.Count) {
    $selected = @($installed[0].Name)
    Write-Host "Using already installed recognizer capability $($selected[0]); the packaged native probe must still confirm recognition readiness."
}
if (-not $selected.Count) {
    $fallback = @($available | Sort-Object @{ Expression = { $_.Name -notlike 'Language.OCR~~~en-US~*' } }, Name | Select-Object -First 1)
    if (-not $fallback.Count) { throw 'OCR_CAPABILITY_CATALOG_EMPTY: Windows exposes no supported native OCR capability; servicing policy or Windows edition must permit OCR.' }
    $selected = @($fallback[0].Name)
    Write-Host "No profile-specific recognizer is offered; provisioning supported fallback $($selected[0]) automatically."
}
foreach ($name in @($selected | Select-Object -Unique)) {
    $state = Get-WindowsCapability -Online -Name $name -ErrorAction Stop
    if ($state.State -ne 'Installed') {
        Write-Host "Installing $name from Windows Update (may take several minutes)..."
        $result = Add-WindowsCapability -Online -Name $name -ErrorAction Stop
        if ($result.RestartNeeded) { throw "OCR_RESTART_REQUIRED: Restart Windows and run setup again ($name)." }
    }
    if ((Get-WindowsCapability -Online -Name $name -ErrorAction Stop).State -ne 'Installed') { throw "OCR_CAPABILITY_NOT_INSTALLED: $name" }
    Write-Host "Installed OCR capability: $name"
}