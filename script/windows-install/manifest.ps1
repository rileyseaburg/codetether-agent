# Full-trust desktop package: the alias activates the packaged app, never a PATH copy.
param([string]$Payload, [string]$Architecture, [string]$Publisher, [string]$Version)
$publisherXml = [Security.SecurityElement]::Escape($Publisher)
@"
<?xml version="1.0" encoding="utf-8"?>
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"
 xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"
 xmlns:uap3="http://schemas.microsoft.com/appx/manifest/uap/windows10/3"
 xmlns:desktop="http://schemas.microsoft.com/appx/manifest/desktop/windows10"
 xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
 IgnorableNamespaces="uap uap3 desktop rescap">
 <Identity Name="CodeTether.Agent.Local" Publisher="$publisherXml" Version="$Version" ProcessorArchitecture="$Architecture" />
 <Properties><DisplayName>CodeTether</DisplayName><PublisherDisplayName>Local CodeTether installation</PublisherDisplayName><Logo>Logo.png</Logo></Properties>
 <Dependencies><TargetDeviceFamily Name="Windows.Desktop" MinVersion="10.0.19041.0" MaxVersionTested="10.0.26100.0" /></Dependencies>
 <Resources><Resource Language="en-us" /></Resources>
 <Applications><Application Id="CodeTether" Executable="codetether.exe" EntryPoint="Windows.FullTrustApplication">
  <uap:VisualElements DisplayName="CodeTether" Description="CodeTether Agent" BackgroundColor="transparent" Square150x150Logo="Logo.png" Square44x44Logo="SmallLogo.png" />
  <Extensions><uap3:Extension Category="windows.appExecutionAlias" Executable="codetether.exe" EntryPoint="Windows.FullTrustApplication">
   <uap3:AppExecutionAlias><desktop:ExecutionAlias Alias="codetether.exe" /></uap3:AppExecutionAlias>
  </uap3:Extension></Extensions>
 </Application></Applications>
 <Capabilities><rescap:Capability Name="runFullTrust" /></Capabilities>
</Package>
"@ | Set-Content (Join-Path $Payload 'AppxManifest.xml') -Encoding UTF8
Add-Type -AssemblyName System.Drawing
foreach ($size in @(150, 44)) {
    $bitmap = New-Object Drawing.Bitmap($size, $size)
    $graphics = [Drawing.Graphics]::FromImage($bitmap)
    try {
        $graphics.Clear([Drawing.Color]::FromArgb(24, 150, 170))
        $name = if ($size -eq 150) { 'Logo.png' } else { 'SmallLogo.png' }
        $bitmap.Save((Join-Path $Payload $name), [Drawing.Imaging.ImageFormat]::Png)
    } finally { $graphics.Dispose(); $bitmap.Dispose() }
}
