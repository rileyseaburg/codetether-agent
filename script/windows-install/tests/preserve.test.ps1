param([string]$Root, [string]$Work)
$previous = Join-Path $Work 'installed-fixture'
New-Item -ItemType Directory $previous | Out-Null
$source = Join-Path $previous 'codetether.exe'
[IO.File]::WriteAllText($source, 'old executable fixture')
$global:previousPackageFixture = [pscustomobject]@{
    Name = 'CodeTether.Agent.Local'; Version = '1.0.0.0'; PackageFamilyName = 'CodeTether.Agent.Local_fixture'
    PackageFullName = 'CodeTether.Agent.Local_1.0.0.0_x64__fixture'; InstallLocation = $previous
}
function Get-AppxPackage { param([string]$Name); $global:previousPackageFixture }
& "$Root/preserve-package.ps1" -Package @{ Name = 'CodeTether.Agent.Local' } -Work $Work
$backup = Join-Path $Work "previous-$($global:previousPackageFixture.PackageFullName)/codetether.exe"
Assert-Contract (Test-Path $source) 'previous installed executable untouched'
Assert-Contract ((Get-FileHash $source).Hash -eq (Get-FileHash $backup).Hash) 'previous packaged executable backup is byte-identical'
