# Freeze verified helper text and public data BEFORE requesting explicit UAC consent.
param([string]$Certificate, [string[]]$Languages)
if ($Certificate -and $Certificate -notmatch '^[A-Za-z0-9+/]+={0,2}$') { throw 'INVALID_CERTIFICATE_ENCODING' }
$data = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes((ConvertTo-Json -InputObject @($Languages | Where-Object { $_ }) -Compress)))
$trust = Get-Content -Raw -LiteralPath "$PSScriptRoot\trust-leaf.ps1"
$ocr = Get-Content -Raw -LiteralPath "$PSScriptRoot\ocr-capabilities.ps1"
Write-Host 'Windows will now request administrator consent (UAC). No app or SDK tool runs elevated.'
if ($Certificate) {
    $cert = New-Object Security.Cryptography.X509Certificates.X509Certificate2(,[Convert]::FromBase64String($Certificate))
    Write-Host "Trust operation: add ONLY code-signing leaf $($cert.Thumbprint), $($cert.Subject), to LocalMachine/TrustedPeople."
    Write-Host 'This machine will trust packages signed by this local user key. No Root CA trust is added; the private key stays in CurrentUser/My.'
    $cert.Dispose()
}
Write-Host "OCR operation: use an installed recognizer or install Windows Language.OCR for this user's languages: $($Languages -join ', ')."
Write-Host 'Developer Mode, execution policy and signature enforcement are not changed. Windows Update/enterprise policy may block capability downloads.'
$command = @"
`$ErrorActionPreference = 'Stop'
try {
    & { $trust } '$Certificate'
    & { $ocr } '$data'
    exit 0
} catch {
    Write-Host ('PREREQUISITE_FAILED: ' + `$_.Exception.Message) -ForegroundColor Red
    Write-Host 'Capability details: C:\Windows\Logs\DISM\dism.log. Existing executable remains untouched.'
    Read-Host 'Press Enter to return to setup'
    exit 1
}
"@
$encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($command))
$powershell = Join-Path $env:SystemRoot 'System32\WindowsPowerShell\v1.0\powershell.exe'
try { $process = Start-Process -FilePath $powershell -Verb RunAs -ArgumentList @('-NoProfile', '-EncodedCommand', $encoded) -Wait -PassThru }
catch { throw "UAC_DENIED_OR_UNAVAILABLE: Prerequisites were not confirmed; setup stopped. $($_.Exception.Message)" }
if ($process.ExitCode -ne 0) { throw "PREREQUISITE_FAILED: Elevated operation exited $($process.ExitCode). See its error window and Windows DISM logs. Setup did not register a package." }
