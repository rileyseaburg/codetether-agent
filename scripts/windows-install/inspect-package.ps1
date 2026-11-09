# Validate the package contract before any registration or privilege request.
param([string]$Path)
$resolved = (Resolve-Path -LiteralPath $Path).Path
Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip = [IO.Compression.ZipFile]::OpenRead($resolved)
try {
    $entry = $zip.GetEntry('AppxManifest.xml')
    if (-not $entry -or $entry.Length -gt 1048576) { throw 'INVALID_MSIX_MANIFEST' }
    $settings = New-Object Xml.XmlReaderSettings
    $settings.DtdProcessing = [Xml.DtdProcessing]::Prohibit; $settings.XmlResolver = $null
    $stream = $entry.Open(); $reader = [Xml.XmlReader]::Create($stream, $settings)
    try { $xml = New-Object Xml.XmlDocument; $xml.XmlResolver = $null; $xml.Load($reader) }
    finally { $reader.Dispose(); $stream.Dispose() }
    $ns = New-Object Xml.XmlNamespaceManager($xml.NameTable)
    $ns.AddNamespace('f', 'http://schemas.microsoft.com/appx/manifest/foundation/windows10')
    $ns.AddNamespace('u', 'http://schemas.microsoft.com/appx/manifest/uap/windows10/3')
    $ns.AddNamespace('d', 'http://schemas.microsoft.com/appx/manifest/desktop/windows10')
    $id = $xml.SelectSingleNode('/f:Package/f:Identity', $ns)
    $app = $xml.SelectSingleNode('/f:Package/f:Applications/f:Application[@Id="CodeTether"]', $ns)
    if (-not $id -or $id.Name -notin @('CodeTether.Agent', 'CodeTether.Agent.Local') -or -not $app) { throw 'UNEXPECTED_PACKAGE_IDENTITY' }
    if ($app.Executable -ne 'codetether.exe' -or $app.EntryPoint -ne 'Windows.FullTrustApplication') { throw 'FULL_TRUST_APPLICATION_REQUIRED' }
    $alias = $app.SelectSingleNode('f:Extensions/u:Extension[@Category="windows.appExecutionAlias"]', $ns)
    if (-not $alias -or $alias.Executable -ne 'codetether.exe' -or $alias.EntryPoint -ne 'Windows.FullTrustApplication' -or
        -not $alias.SelectSingleNode('u:AppExecutionAlias/d:ExecutionAlias[@Alias="codetether.exe"]', $ns)) { throw 'PACKAGE_ALIAS_REQUIRED' }
    @{ Path = $resolved; Name = [string]$id.Name; Publisher = [string]$id.Publisher; Version = [string]$id.Version }
} finally { $zip.Dispose() }
