param([string]$Root, [string]$Work)
Add-Type -AssemblyName System.IO.Compression.FileSystem
$template = Get-Content "$Root/manifest.ps1" -Raw
$xml = [regex]::Match($template, '(?s)<\?xml.*?</Package>').Value
$xml = $xml.Replace('$publisherXml', 'CN=CodeTether Local S-1-5-21-123').Replace('$Version', '1.0.0.0').Replace('$Architecture', 'x64')
$directory = Join-Path $Work 'package-fixture'
New-Item -ItemType Directory $directory | Out-Null
$manifest = Join-Path $directory 'AppxManifest.xml'
$xml | Set-Content $manifest
$path = Join-Path $Work 'fixture.msix'
[IO.Compression.ZipFile]::CreateFromDirectory($directory, $path)
$result = & "$Root/inspect-package.ps1" -Path $path
Assert-Contract ($result.Name -eq 'CodeTether.Agent.Local') 'local package identity recognized'
$xml.Replace('Alias="codetether.exe"', 'Alias="wrong.exe"') | Set-Content $manifest
$invalid = Join-Path $Work 'bad-alias.msix'
[IO.Compression.ZipFile]::CreateFromDirectory($directory, $invalid)
Assert-Throws { & "$Root/inspect-package.ps1" -Path $invalid } 'PACKAGE_ALIAS_REQUIRED'
function Add-AppxPackage { param($Path, $ErrorAction); throw 'fixture registration rejection' }
Assert-Throws { & "$Root/register.ps1" -Package $result -Work $Work } 'PACKAGE_REGISTRATION_FAILED'
$register = Get-Content "$Root/register.ps1" -Raw
Assert-Contract ($register.IndexOf('Add-AppxPackage') -lt $register.IndexOf('& $alias --version')) 'register before activation'
Assert-Contract ($register.IndexOf('& $alias --version') -lt $register.IndexOf('publish-alias.ps1')) 'activate before changing legacy install/PATH'
$build = Get-Content "$Root/build-package.ps1" -Raw
Assert-Contract ($build -match 'sign /fd SHA256 /sha1 \$cert.Thumbprint /s My') 'CurrentUser certificate selected by thumbprint'
$sign = (Get-Content "$Root/build-package.ps1" | Where-Object { $_ -match 'signtool.exe.*sign' }) -join ''
Assert-Contract ($sign -notmatch '/p\s|Export-PfxCertificate') 'no exported signing key/password'
