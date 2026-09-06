param([string]$Root, [string]$Work)
# Compile the reparse-tag interop without invoking kernel32 on this host.
$source = Get-Content "$Root/alias-tag.ps1" -Raw
$csharp = [regex]::Match($source, "(?s)@'(.*?)'@").Groups[1].Value
Assert-Contract (-not [string]::IsNullOrWhiteSpace($csharp)) 'alias tag interop source exists'
Add-Type -TypeDefinition $csharp
Assert-Contract ($null -ne ('CodeTether.InstallAlias' -as [type])) 'alias tag interop compiles'
Assert-Contract ($csharp -match '0x8000001b') 'requires AppExecLink tag, not merely a reparse point'
$register = Get-Content "$Root/register.ps1" -Raw
$resolver = Get-Content "$Root/resolve-alias.ps1" -Raw
Assert-Contract ($resolver.Contains('Microsoft\WindowsApps\$($Registered.PackageFamilyName)')) 'family-scoped alias selected'
Assert-Contract ($register.IndexOf('alias-tag.ps1') -lt $register.IndexOf('& $alias --version')) 'tag checked before activation'
