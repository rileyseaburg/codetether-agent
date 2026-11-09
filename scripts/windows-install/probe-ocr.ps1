# Require explicit native identity and actual recognition proof, without a model or Vault.
param([string]$Alias, [string]$Work)
$statusPath = Join-Path $Work 'ocr-status.json'
$errorPath = Join-Path $Work 'ocr-status.stderr.log'
try { $exitCode = & "$PSScriptRoot\run-ocr-probe.ps1" -Alias $Alias -Work $Work }
catch { throw "OCR_PROBE_FAILED: $($_.Exception.Message) See $statusPath and $errorPath. The legacy executable is preserved." }
if ($exitCode -ne 0) { throw "OCR_NOT_READY: Packaged readiness probe exited $exitCode. See $statusPath and $errorPath. The legacy executable is preserved." }
try { $status = Get-Content -LiteralPath $statusPath -Raw | ConvertFrom-Json -ErrorAction Stop }
catch { throw "OCR_STATUS_INVALID: Native probe did not emit valid JSON. See $statusPath." }
foreach ($field in @('available', 'package_identity_present', 'recognition_probe_succeeded')) {
    if ($null -eq $status -or $null -eq $status.PSObject.Properties[$field] -or
        $status.$field -isnot [bool] -or -not $status.$field) {
        throw "OCR_NOT_READY: Expected native status $field=true, even with exit code zero. See $statusPath."
    }
}
Write-Host "Native package identity and recognition readiness confirmed: $statusPath"