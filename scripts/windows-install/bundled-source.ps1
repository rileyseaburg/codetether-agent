# Resolve inputs beside install.ps1 before considering network release discovery.
param([string]$Directory)
$msix = Join-Path $Directory 'codetether.msix'
$exe = Join-Path $Directory 'codetether.exe'
if (Test-Path -LiteralPath $msix -PathType Leaf) { return @{ Msix = $msix; Exe = '' } }
if (Test-Path -LiteralPath $exe -PathType Leaf) { return @{ Msix = ''; Exe = $exe } }
@{ Msix = ''; Exe = '' }
