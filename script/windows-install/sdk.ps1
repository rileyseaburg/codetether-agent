# Pinned SDK bytes downloaded from NuGet; no developer toolchain needed.
param([string]$Work)
$version = '10.0.26100.3916'
$sha512 = '3dca54ecefe7a7366816bbc37668e8a5b3588c918b5e164cbaee7beda2bf8b558fc83083fe02da64d3c9701181f7edca097f0b51b8cbe677a649a29676680332'
$uri = "https://api.nuget.org/v3-flatcontainer/microsoft.windows.sdk.buildtools/$version/microsoft.windows.sdk.buildtools.$version.nupkg"
$archive = Join-Path $Work 'sdk.zip'
try { Invoke-WebRequest $uri -OutFile $archive -UseBasicParsing }
catch { throw "SDK_UNAVAILABLE: Could not download pinned Microsoft SDK $version. $($_.Exception.Message)" }
if ((Get-FileHash $archive -Algorithm SHA512).Hash -ine $sha512) { throw 'SDK_INTEGRITY_FAILED: SHA512 mismatch; refusing to execute tools.' }
$root = Join-Path $Work 'sdk'
Expand-Archive $archive $root
$native = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
$arch = switch ($native) { 'AMD64' { 'x64' } 'ARM64' { 'arm64' } default { throw "SDK_ARCH_UNSUPPORTED: $native" } }
$bin = Join-Path $root "bin\10.0.26100.0\$arch"
foreach ($tool in @('makeappx.exe', 'signtool.exe')) {
    if (-not (Test-Path (Join-Path $bin $tool))) { throw "SDK_TOOL_MISSING: $tool" }
}
$bin
