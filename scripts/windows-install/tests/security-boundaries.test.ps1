param([string]$Root, [string]$Work)
# Static contracts complement the mocked runtime tests; they do not establish Windows behavior.
$certificate = Get-Content "$Root/certificate.ps1" -Raw
$keyPolicy = Get-Content "$Root/key-policy.ps1" -Raw
$trust = Get-Content "$Root/trust-leaf.ps1" -Raw
$publish = Get-Content "$Root/publish-alias.ps1" -Raw
$register = Get-Content "$Root/register.ps1" -Raw
$all = (Get-ChildItem $Root -Filter '*.ps1' -File | ForEach-Object { Get-Content $_.FullName -Raw }) -join "`n"
Assert-Contract ($certificate -match '-CertStoreLocation Cert:\\CurrentUser\\My -KeyExportPolicy NonExportable') 'new signing key stays nonexportable in CurrentUser/My'
Assert-Contract ($certificate -match 'SIGNING_KEY_POLICY_REJECTED' -and $keyPolicy -match 'CngExportPolicies\]::None') 'created and reused key export policy is checked'
Assert-Contract ($trust -match "X509Store\('TrustedPeople', 'LocalMachine'\)") 'leaf trust store fixed to LocalMachine/TrustedPeople'
Assert-Contract ($all -notmatch 'Export-PfxCertificate|Set-ExecutionPolicy|EnableDeveloperMode|AllowAllTrustedApps|/sm\s') 'no PFX, machine signing key store, or system policy bypass'
Assert-Contract ($publish.Contains('$env:PATH = "$AliasDir;$oldPath"')) 'family alias prepended before old bare bin in current PATH'
Assert-Contract ($publish.Contains('(@($AliasDir) + $remaining -join')) 'family alias prepended in persisted user PATH'
Assert-Contract ($publish.IndexOf('Get-Command codetether.exe') -lt $publish.IndexOf('Move-Item')) 'command resolution checked before retiring legacy binary'
Assert-Contract ($register.IndexOf('probe-ocr.ps1') -lt $register.IndexOf('publish-alias.ps1')) 'packaged native readiness gates legacy retirement'
