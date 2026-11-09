# .NET process capture avoids PowerShell 5.1 converting native stderr into errors.
param([string]$Alias, [string]$Work, [string]$Arguments = 'windows ocr-status --require-ready')
$stdout = Join-Path $Work 'ocr-status.json'
$stderr = Join-Path $Work 'ocr-status.stderr.log'
$encoding = New-Object Text.UTF8Encoding($false)
[IO.File]::WriteAllText($stdout, '', $encoding)
[IO.File]::WriteAllText($stderr, '', $encoding)
$start = New-Object Diagnostics.ProcessStartInfo
$start.FileName = $Alias
$start.Arguments = $Arguments
$start.WorkingDirectory = (Resolve-Path -LiteralPath $Work).Path
$start.UseShellExecute = $false
$start.CreateNoWindow = $true
$start.RedirectStandardOutput = $true; $start.RedirectStandardError = $true
$start.StandardOutputEncoding = $encoding; $start.StandardErrorEncoding = $encoding
$process = New-Object Diagnostics.Process
$process.StartInfo = $start
$outTask = $null; $errTask = $null
try {
    if (-not $process.Start()) { throw 'OCR_PROCESS_NOT_STARTED' }
    $outTask = $process.StandardOutput.ReadToEndAsync()
    $errTask = $process.StandardError.ReadToEndAsync()
    if (-not $process.WaitForExit(120000)) {
        $process.Kill(); $process.WaitForExit()
        throw 'OCR_PROBE_TIMEOUT: Native readiness did not finish within 120 seconds.'
    }
    $process.WaitForExit()
    $outTask.Wait(); $errTask.Wait()
    $process.ExitCode
} finally {
    if ($null -ne $outTask) {
        $outTask.Wait(); [IO.File]::WriteAllText($stdout, $outTask.Result, $encoding)
    }
    if ($null -ne $errTask) {
        $errTask.Wait(); [IO.File]::WriteAllText($stderr, $errTask.Result, $encoding)
    }
    $process.Dispose()
}