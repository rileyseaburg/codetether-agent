# Read PE architecture without executing an uninstalled/bare application.
param([string]$Path)
$resolved = (Resolve-Path -LiteralPath $Path).Path
$stream = [IO.File]::OpenRead($resolved)
$reader = New-Object IO.BinaryReader($stream)
try {
    if ($reader.ReadUInt16() -ne 0x5a4d) { throw 'INVALID_EXE: Missing MZ header.' }
    $stream.Position = 0x3c; $offset = $reader.ReadInt32()
    if ($offset -lt 0 -or $offset -gt ($stream.Length - 6)) { throw 'INVALID_EXE: Invalid PE offset.' }
    $stream.Position = $offset
    if ($reader.ReadUInt32() -ne 0x4550) { throw 'INVALID_EXE: Missing PE signature.' }
    $arch = switch ($reader.ReadUInt16()) { 0x8664 { 'x64' } 0xaa64 { 'arm64' } default { throw 'EXE_ARCH_UNSUPPORTED: Requires x64 or ARM64.' } }
} finally { $reader.Dispose(); $stream.Dispose() }
@{ Path = $resolved; Architecture = $arch }
