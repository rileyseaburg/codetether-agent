param([string]$Root, [string]$Work)
$text = Get-Content -LiteralPath (Join-Path $Root 'Diagnose-CodeTether-Install.cmd') -Raw
Assert-Contract ($text.Contains('DisableDelayedExpansion')) 'special characters in paths are not expanded'
Assert-Contract ($text.Contains('/L*V! "%REPORT%\msi.log"')) 'MSI verbose logging is enabled and flushed'
Assert-Contract ($text.Contains('/i "%REPORT%\codetether-windows.msi"')) 'MSI runs from a local copy'
Assert-Contract ($text.Contains('exit-code.txt')) 'installer exit code is retained'
Assert-Contract ($text.Contains('codetether-windows\errors\%RUNID%')) 'diagnostics return to a unique NAS folder'
Assert-Contract ($text.Contains('*.log *.txt *.json')) 'only diagnostics return to NAS, not the MSI or secrets'
Assert-Contract ($text -notmatch 'ExecutionPolicy|Set-ExecutionPolicy|reg add|VAULT_TOKEN') 'diagnostic wrapper does not change policy or inspect credentials'
Assert-Contract ($text.Contains('failure.txt makeappx.log signtool.log ocr-status.json')) 'only known setup diagnostics are collected'
Assert-Contract ($text.Contains('exit /b %INSTALL_RESULT%')) 'the original installer result is returned'
