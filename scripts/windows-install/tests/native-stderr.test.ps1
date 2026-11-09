# Real child-process I/O: stderr warnings must not become PowerShell terminating errors.
param([string]$Root, [string]$Work)
$fixture = Join-Path $Work 'native-warning.ps1'
@'
[Console]::Error.WriteLine('fixture warning: telemetry unavailable')
[Console]::Out.WriteLine('{"available":true,"package_identity_present":true,"recognition_probe_succeeded":true}')
[Console]::Out.WriteLine('')
exit 0
'@ | Set-Content -LiteralPath $fixture
$binary = (Get-Process -Id $PID).Path
$directory = Join-Path $Work 'native-stderr'
New-Item -ItemType Directory -Path $directory | Out-Null
$exitCode = & "$Root/run-ocr-probe.ps1" -Alias $binary -Work $directory -Arguments ('-NoLogo -NoProfile -File "' + $fixture + '"')
Assert-Contract ($exitCode -eq 0) 'native warning is not a process failure'
$status = Get-Content (Join-Path $directory 'ocr-status.json') -Raw | ConvertFrom-Json
Assert-Contract ($status.available -eq $true) 'JSON stdout survives stderr warning'
Assert-Contract ((Get-Content (Join-Path $directory 'ocr-status.stderr.log') -Raw).Contains('fixture warning')) 'stderr evidence is retained, not discarded'
@'
[Console]::Error.WriteLine('fixture native error')
exit 7
'@ | Set-Content -LiteralPath $fixture
$exitCode = & "$Root/run-ocr-probe.ps1" -Alias $binary -Work $directory -Arguments ('-NoLogo -NoProfile -File "' + $fixture + '"')
Assert-Contract ($exitCode -eq 7) 'nonzero native exit code is preserved'
@'
[Console]::Out.WriteLine((Get-Location).Path)
exit 0
'@ | Set-Content -LiteralPath $fixture
& "$Root/run-ocr-probe.ps1" -Alias $binary -Work $directory -Arguments ('-NoLogo -NoProfile -File "' + $fixture + '"') | Out-Null
$cwd = (Get-Content (Join-Path $directory 'ocr-status.json') -Raw).Trim()
Assert-Contract ($cwd -eq (Resolve-Path $directory).Path) 'probe runs in writable work directory, not System32'
