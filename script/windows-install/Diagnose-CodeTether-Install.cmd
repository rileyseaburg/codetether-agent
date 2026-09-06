@echo off
setlocal DisableDelayedExpansion
set "NASROOT=%~dp0"
set "RUNID=run-%RANDOM%-%RANDOM%"
set "REPORT=%LOCALAPPDATA%\codetether\install-diagnostics\%RUNID%"
set "NASREPORT=%NASROOT%codetether-windows\errors\%RUNID%"
cd /d "%SystemRoot%"
echo Preparing a local copy and verbose Windows Installer diagnostics.
mkdir "%REPORT%" 2>nul
if not exist "%REPORT%\" goto preparation_failed
copy /b /y "%NASROOT%codetether-windows.msi" "%REPORT%\codetether-windows.msi" >nul
if errorlevel 1 goto preparation_failed
echo Local diagnostic directory: %REPORT%
echo Starting CodeTether setup. Close its result dialog when it finishes.
start "" /wait "%SystemRoot%\System32\msiexec.exe" /i "%REPORT%\codetether-windows.msi" /L*V! "%REPORT%\msi.log"
set "INSTALL_RESULT=%ERRORLEVEL%"
echo Windows Installer exit code: %INSTALL_RESULT%
echo %INSTALL_RESULT%>"%REPORT%\exit-code.txt"
ver >"%REPORT%\windows-version.txt"
echo %PROCESSOR_ARCHITECTURE%>>"%REPORT%\windows-version.txt"
rem Copy only known diagnostics, never Vault settings, keys, certificates, or SDK payloads.
robocopy "%LOCALAPPDATA%\codetether\install-evidence" "%REPORT%\setup-evidence" failure.txt makeappx.log signtool.log ocr-status.json ocr-status.stderr.log alias-repair.log activation.log msi-entry.log /S /R:0 /W:0 /XD sdk payload /NFL /NDL /NJH /NJS >nul 2>&1
mkdir "%NASREPORT%" 2>nul
robocopy "%REPORT%" "%NASREPORT%" *.log *.txt *.json /E /R:0 /W:0 /NFL /NDL /NJH /NJS >nul 2>&1
if errorlevel 8 goto copy_failed
echo Diagnostics copied to: %NASREPORT%
echo No execution or signing policies were changed by this diagnostic launcher.
pause
exit /b %INSTALL_RESULT%
:copy_failed
echo Could not copy diagnostics to the NAS. They are retained at: %REPORT%
pause
exit /b %INSTALL_RESULT%
:preparation_failed
echo Could not stage the MSI. Keep this launcher beside codetether-windows.msi on the NAS.
pause
exit /b 1