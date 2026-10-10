# Runs only Rust migration libraries, including deterministic scheduling policy.
# Never launches OS capture, networking, the GUI application, or .NET reference.
# Copy the source archive to Downloads/companion-source-verification.tar.gz first.
param([string]$Archive = "$env:USERPROFILE/Downloads/companion-source-verification.tar.gz")
$ErrorActionPreference = 'Stop'
$root = Join-Path $env:TEMP ("codetether-companion-tests-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $root | Out-Null
Write-Output "Evidence directory: $root"
Get-FileHash -Algorithm SHA256 $Archive
Push-Location $root
try {
    & tar -xzf $Archive
    if ($LASTEXITCODE -ne 0) { throw 'Source archive extraction failed' }
    # Preserve the repository lockfile before reducing the workspace membership.
    Copy-Item Cargo.lock Cargo.workspace.lock
    @'
[workspace]
members = ["crates/*"]
resolver = "3"
'@ | Set-Content -Encoding ASCII Cargo.toml
    # The reduced workspace must prune the repository lockfile. Retain both.
    $ErrorActionPreference = 'Continue'
    & cmd /c 'cargo test --workspace > windows-tests.log 2>&1'
    $code = $LASTEXITCODE
    Get-Content windows-tests.log
    Get-FileHash -Algorithm SHA256 Cargo.lock
    Write-Output "Tests exit code: $code"
    if ($code -ne 0) { exit $code }
    & cmd /c 'cargo clippy --workspace --all-targets --locked -- -D warnings > windows-clippy.log 2>&1'
    $code = $LASTEXITCODE
    Get-Content windows-clippy.log
    Write-Output "Clippy exit code: $code"
    exit $code
}
finally {
    Pop-Location
}