param([string]$Root, [string]$Work)
$path = Join-Path $Work 'fixture.exe'
$bytes = New-Object byte[] 128
$bytes[0] = 0x4d; $bytes[1] = 0x5a; $bytes[0x3c] = 0x40
$bytes[0x40] = 0x50; $bytes[0x41] = 0x45
$bytes[0x44] = 0x64; $bytes[0x45] = 0x86
[IO.File]::WriteAllBytes($path, $bytes)
$result = & "$Root/executable.ps1" -Path $path
Assert-Contract ($result.Architecture -eq 'x64') 'x64 PE recognition'
$bytes[0x44] = 0x64; $bytes[0x45] = 0xaa
[IO.File]::WriteAllBytes($path, $bytes)
$result = & "$Root/executable.ps1" -Path $path
Assert-Contract ($result.Architecture -eq 'arm64') 'ARM64 PE recognition'
$bytes[0] = 0
[IO.File]::WriteAllBytes($path, $bytes)
Assert-Throws { & "$Root/executable.ps1" -Path $path } 'INVALID_EXE'
