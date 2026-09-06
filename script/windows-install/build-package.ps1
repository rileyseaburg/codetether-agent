# Wrap a local/downloaded executable and adjacent native DLLs in a full-trust MSIX.
param([string]$ExePath, [string]$Work)
$exe = & "$PSScriptRoot\executable.ps1" -Path $ExePath
$sdk = & "$PSScriptRoot\sdk.ps1" -Work $Work
$cert = & "$PSScriptRoot\certificate.ps1"
[IO.File]::WriteAllBytes((Join-Path $Work 'signing-leaf.cer'), $cert.RawData)
$cert | Format-List Subject, Thumbprint, NotAfter | Out-String | Set-Content (Join-Path $Work 'signing-leaf.txt')
$payload = Join-Path $Work 'payload'
New-Item -ItemType Directory $payload | Out-Null
Copy-Item -LiteralPath $exe.Path -Destination (Join-Path $payload 'codetether.exe')
Get-ChildItem -LiteralPath (Split-Path $exe.Path) -Filter '*.dll' -File | Copy-Item -Destination $payload
# Seconds-based version supports repeated local development installs.
$seconds = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds() - 1577836800
$version = "1.0.$([math]::Floor($seconds / 65535)).$($seconds % 65535)"
& "$PSScriptRoot\manifest.ps1" -Payload $payload -Architecture $exe.Architecture -Publisher $cert.Subject -Version $version
$path = Join-Path $Work 'codetether-local.msix'
& "$sdk\makeappx.exe" pack /d $payload /p $path /o *> (Join-Path $Work 'makeappx.log')
if ($LASTEXITCODE -ne 0) { throw "MSIX_BUILD_FAILED: See $Work\makeappx.log" }
# /sha1 selects CurrentUser\My. Never export a PFX or pass a password.
& "$sdk\signtool.exe" sign /fd SHA256 /sha1 $cert.Thumbprint /s My $path *> (Join-Path $Work 'signtool.log')
if ($LASTEXITCODE -ne 0) { throw "MSIX_SIGN_FAILED: See $Work\signtool.log" }
@{ Path = $path; Certificate = [Convert]::ToBase64String($cert.RawData) }
