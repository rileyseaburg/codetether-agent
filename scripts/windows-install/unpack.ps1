# Extract executable assets into retained staging; reject link/traversal tar entries.
param([string]$Path, [string]$Work)
if ($Path.EndsWith('.exe')) { return $Path }
$directory = Join-Path $Work 'release'
New-Item -ItemType Directory $directory | Out-Null
if ($Path.EndsWith('.zip')) { Expand-Archive -LiteralPath $Path -DestinationPath $directory }
elseif ($Path.EndsWith('.tar.gz')) {
    $tar = Join-Path $env:SystemRoot 'System32\tar.exe'
    if (-not (Test-Path $tar)) { throw 'TAR_UNAVAILABLE: Supply an extracted -ExePath instead.' }
    $entries = @(& $tar -tzf $Path)
    if ($LASTEXITCODE -ne 0) { throw 'ARCHIVE_LIST_FAILED' }
    foreach ($entry in $entries) {
        if ($entry -match '(^[/\\]|^[A-Za-z]:|(^|[/\\])\.\.([/\\]|$))') { throw 'UNSAFE_ARCHIVE_PATH' }
    }
    $details = @(& $tar -tvzf $Path)
    if ($LASTEXITCODE -ne 0 -or @($details | Where-Object { $_ -notmatch '^[-d]' }).Count) { throw 'UNSAFE_ARCHIVE_ENTRY: Only files and directories are allowed.' }
    & $tar -xzf $Path -C $directory
    if ($LASTEXITCODE -ne 0) { throw 'ARCHIVE_EXTRACTION_FAILED' }
} else { throw 'UNSUPPORTED_ARCHIVE_FORMAT' }
$executables = @(Get-ChildItem $directory -Filter 'codetether.exe' -Recurse -File)
if ($executables.Count -ne 1) { throw 'ARCHIVE_EXE_AMBIGUOUS: Expected exactly one codetether.exe.' }
$executables[0].FullName
