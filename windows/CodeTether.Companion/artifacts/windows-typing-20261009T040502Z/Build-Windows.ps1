# Build only the Rust companion, using the unmodified repository workspace.
param([string]$Archive, [string]$Destination, [string]$Attempt = '01')
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
if (!(Test-Path $Destination)) {
    New-Item -ItemType Directory -Path $Destination | Out-Null
    & tar.exe -xf $Archive -C $Destination
    if ($LASTEXITCODE -ne 0) { throw 'Source extraction failed' }
    Get-FileHash -Algorithm SHA256 $Archive | Format-List | Out-File "$Destination\source-hash.txt"
}
$manifest = Join-Path $Destination 'Cargo.toml'
if (!(Test-Path $manifest)) { throw 'Repository root Cargo.toml is missing' }
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$vs = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (!$vs) { throw 'Visual C++ x64 build tools are unavailable' }
$devcmd = Join-Path $vs 'Common7\Tools\VsDevCmd.bat'
$cargo = Join-Path $HOME '.cargo\bin\cargo.exe'
$env:CARGO_TARGET_DIR = Join-Path $Destination 'target'
# Avoid the Linux checkout's machine-specific Mold linker flag; use MSVC.
$env:RUSTFLAGS = '-Cdebuginfo=0'
$env:CMAKE_GENERATOR = 'NMake Makefiles'
$env:CARGO_TERM_COLOR = 'never'
$log = Join-Path $Destination "build-$Attempt.log"
if (Test-Path $log) { throw 'Preserve existing build logs: use a new attempt number' }
Push-Location $Destination
try {
    $command = 'call "{0}" -arch=x64 -host_arch=x64 >nul && "{1}" +stable build --locked -p codetether-companion-shell --bin codetether-companion >"{2}" 2>&1' -f $devcmd, $cargo, $log
    & cmd.exe /d /s /c $command
    $code = $LASTEXITCODE
    Set-Content -Path "$Destination\build-$Attempt.exit.txt" -Value $code
    Get-Content $log -Tail 90
    if ($code -eq 0) { Get-FileHash -Algorithm SHA256 "$Destination\target\debug\codetether-companion.exe" | Format-List }
    exit $code
} finally { Pop-Location }
