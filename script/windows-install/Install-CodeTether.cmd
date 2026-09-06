@echo off
setlocal DisableDelayedExpansion
set "CODETETHER_BUNDLE_DIR=%~dp0"
set "CODETETHER_PARENT_EXECUTION_POLICY=%PSExecutionPolicyPreference%"
rem Do not map the share: elevated processes must only receive local paths.
cd /d "%SystemRoot%"
echo Staging this trusted extracted bundle locally before installing CodeTether.
echo Windows may request consent for local package trust and OCR language support.
"%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -ExecutionPolicy RemoteSigned -Command "$ErrorActionPreference='Stop'; $code=0; $logging=$false; try { $blocked=@(Get-ExecutionPolicy -List | Where-Object { $_.ExecutionPolicy -eq 'AllSigned' -or ($_.Scope -in @('MachinePolicy','UserPolicy') -and $_.ExecutionPolicy -eq 'Restricted') }); if($blocked.Count -or $env:CODETETHER_PARENT_EXECUTION_POLICY -eq 'AllSigned'){throw 'SIGNED_INSTALLER_REQUIRED: Organization/signing policy is preserved.'}; $local=$env:LOCALAPPDATA; if(!$local -or $local -notmatch '^[A-Za-z]:\\' -or ([IO.DriveInfo]::new($local)).DriveType -ne 'Fixed'){throw 'LOCAL_STAGE_REQUIRED: LOCALAPPDATA must be on a local fixed drive.'}; $stage=Join-Path $local ('codetether\install-stages\'+[guid]::NewGuid()); New-Item -ItemType Directory -Path $stage | Out-Null; Write-Host ('Local bundle and evidence retained at '+$stage); Start-Transcript -Path (Join-Path $stage 'bootstrap.log') | Out-Null; $logging=$true; $root=$env:CODETETHER_BUNDLE_DIR; $helpers=Join-Path $stage 'script\windows-install'; New-Item -ItemType Directory -Path $helpers -Force | Out-Null; $source=Join-Path $root 'script\windows-install\stage-bundle.ps1'; if(!(Test-Path -LiteralPath $source -PathType Leaf)){throw 'BUNDLE_INCOMPLETE: Missing stage-bundle.ps1; extract the entire ZIP.'}; $bootstrap=Join-Path $helpers 'stage-bundle.ps1'; $hash=(Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash; Copy-Item -LiteralPath $source -Destination $bootstrap; if((Get-FileHash -LiteralPath $bootstrap -Algorithm SHA256).Hash -ne $hash){throw 'BUNDLE_INTEGRITY_FAILED: stage-bundle.ps1'}; Unblock-File -LiteralPath $bootstrap; & $bootstrap -Source $root -Destination $stage; $installer=Join-Path $stage 'install.ps1'; Unblock-File -LiteralPath $installer; Get-ChildItem -LiteralPath $helpers -Filter '*.ps1' -File | Unblock-File; Set-Location -LiteralPath $stage; & $installer -ExePath (Join-Path $stage 'codetether.exe') } catch { $code=1; Write-Error $_.Exception.Message -ErrorAction Continue } finally { if($logging){Stop-Transcript | Out-Null} }; exit $code"
set "RESULT=%ERRORLEVEL%"
if not "%RESULT%"=="0" (
  echo Setup did not finish. The preceding diagnostic and installer evidence explain why.
  pause
)
exit /b %RESULT%
