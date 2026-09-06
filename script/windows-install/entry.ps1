# Entry point shared by the bootstrap and bundled Windows archives.
param([string]$ExePath, [string]$MsixPath, [string]$Version,
    [switch]$FunctionGemma, [switch]$FunctionGemmaOnly, [switch]$Force)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($env:OS -ne 'Windows_NT') { throw 'WINDOWS_REQUIRED' }
if ([Environment]::OSVersion.Version.Build -lt 19041) { throw 'WINDOWS_VERSION_UNSUPPORTED: Requires Windows 10 build 19041 or newer.' }
if ($PSVersionTable.PSEdition -ne 'Desktop' -or -not [Environment]::Is64BitProcess) {
    & "$PSScriptRoot\desktop-host.ps1" -Options $PSBoundParameters; return
}
if ($ExePath -and $MsixPath) { throw 'INVALID_INPUT: Choose -ExePath or -MsixPath, not both.' }
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($identity)
if ($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw 'PER_USER_REQUIRED: Run from a normal, non-elevated PowerShell; setup requests narrowly scoped UAC itself.'
}
$work = Join-Path $env:LOCALAPPDATA "codetether\install-evidence\$([guid]::NewGuid())"
New-Item -ItemType Directory $work -Force | Out-Null
Write-Host "Installation evidence retained at $work. Existing credentials are not changed."
try {
    if ($FunctionGemma -or $FunctionGemmaOnly) { & "$PSScriptRoot\functiongemma.ps1" }
    if ($FunctionGemmaOnly) { return }
    $languages = @(Get-WinUserLanguageList | ForEach-Object { $_.LanguageTag })
    if (-not $languages.Count) { Write-Warning 'No profile languages found; setup will require an already installed OCR recognizer.' }
    if (-not $ExePath -and -not $MsixPath) {
        $source = & "$PSScriptRoot\release.ps1" -Version $Version -Work $work
        $ExePath = $source.Exe; $MsixPath = $source.Msix
    }
    $certificate = ''
    if ($ExePath) {
        $built = & "$PSScriptRoot\build-package.ps1" -ExePath $ExePath -Work $work
        $MsixPath = $built.Path; $certificate = $built.Certificate
    }
    $package = & "$PSScriptRoot\inspect-package.ps1" -Path $MsixPath
    & "$PSScriptRoot\preserve-package.ps1" -Package $package -Work $work
    & "$PSScriptRoot\elevate.ps1" -Certificate $certificate -Languages $languages
    & "$PSScriptRoot\register.ps1" -Package $package -Work $work
    & "$PSScriptRoot\setup-vault.ps1" -CodetetherPath (Get-Command codetether.exe -CommandType Application -ErrorAction Stop).Source
    Write-Host 'Package registration, alias activation, and native OCR recognition probe succeeded.'
    Write-Host 'Packaged probe confirmed available=true, package_identity_present=true, and recognition_probe_succeeded=true without a model or Vault.'
} catch {
    $_ | Out-String | Set-Content (Join-Path $work 'failure.txt')
    Write-Error "INSTALL_FAILED: $($_.Exception.Message) Evidence: $work" -ErrorAction Continue
    throw
}