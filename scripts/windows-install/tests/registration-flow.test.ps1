param([string]$Root, [string]$Work)
# Execute the real registration flow/probe; only OS registration, alias lookup/tag and publication are fixtures.
$directory = Join-Path $Work 'registration-flow'
New-Item -ItemType Directory $directory | Out-Null
Copy-Item "$Root/register.ps1", "$Root/probe-ocr.ps1" $directory
@'
param($Alias,$Work)
[IO.File]::WriteAllText((Join-Path $Work 'ocr-status.json'), $global:registrationFlow.Json)
[IO.File]::WriteAllText((Join-Path $Work 'ocr-status.stderr.log'), 'fixture warning')
$global:registrationFlow.Exit
'@ | Set-Content (Join-Path $directory 'run-ocr-probe.ps1')
'param($Registered,$Work); @{ Directory = $global:registrationFlow.Directory; Path = "Invoke-RegistrationAliasFixture" }' | Set-Content (Join-Path $directory 'resolve-alias.ps1')
'param($Path); Assert-Contract ($Path -eq "Invoke-RegistrationAliasFixture") "resolved alias tagged"' | Set-Content (Join-Path $directory 'alias-tag.ps1')
'param($AliasDir,$Work); $global:registrationFlow.Published++; Assert-Contract ($AliasDir -eq $global:registrationFlow.Directory) "published resolved alias directory"; Move-Item $global:registrationFlow.Legacy (Join-Path $Work "retired-fixture.exe")' | Set-Content (Join-Path $directory 'publish-alias.ps1')
$package = @{ Name = 'CodeTether.Agent.Local'; Path = 'fixture.msix'; Publisher = 'CN=Fixture'; Version = '1.0.0.0' }
function Add-AppxPackage { param($Path,$ErrorAction); $global:registrationFlow.Registered++ }
function Get-AppxPackage {
    param($Name)
    [pscustomobject]@{ Name = $Name; Publisher = 'CN=Fixture'; Version = [version]'1.0.0.0'; PackageFullName = 'fixture'; PackageFamilyName = 'fixture'; Status = 'Ok' }
}
function Invoke-RegistrationAliasFixture {
    if (($args -join ' ') -eq '--version') { $global:LASTEXITCODE = 0; 'codetether fixture'; return }
    Assert-Contract (($args -join ' ') -eq 'windows ocr-status --require-ready') 'registration uses native readiness CLI'
    $global:LASTEXITCODE = $global:registrationFlow.Exit
    $global:registrationFlow.Json
}
foreach ($case in (& "$PSScriptRoot/readiness-cases.ps1")) {
    $caseWork = Join-Path $directory $case.Name
    New-Item -ItemType Directory $caseWork | Out-Null
    $legacy = Join-Path $caseWork 'old-executable.exe'
    [IO.File]::WriteAllText($legacy, 'existing install fixture')
    $global:registrationFlow = @{ Directory = $directory; Published = 0; Registered = 0; Json = $case.Json; Exit = $case.Exit; Legacy = $legacy }
    $action = { & "$directory/register.ps1" -Package $package -Work $caseWork }
    if ($case.Error) {
        Assert-Throws $action $case.Error
        Assert-Contract ($global:registrationFlow.Published -eq 0) "$($case.Name): failed readiness never calls publish-alias"
        Assert-Contract (Test-Path $legacy) "$($case.Name): old executable preserved"
    } else {
        & $action
        Assert-Contract ($global:registrationFlow.Published -eq 1) 'ready status permits publication exactly once'
        Assert-Contract (Test-Path (Join-Path $caseWork 'retired-fixture.exe')) 'retirement fixture reached only after ready status'
    }
    Assert-Contract ($global:registrationFlow.Registered -eq 1) 'readiness evaluated after package registration'
    Assert-Contract (Test-Path (Join-Path $caseWork 'ocr-status.json')) 'registration retains native probe JSON'
}