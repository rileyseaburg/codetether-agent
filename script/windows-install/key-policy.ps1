# Reuse only a private RSA key whose Windows provider confirms export is disabled.
param([Security.Cryptography.X509Certificates.X509Certificate2]$Certificate)
$key = $null
try {
    $key = [Security.Cryptography.X509Certificates.RSACertificateExtensions]::GetRSAPrivateKey($Certificate)
    if ($key -is [Security.Cryptography.RSACng]) {
        return $key.Key.ExportPolicy -eq [Security.Cryptography.CngExportPolicies]::None
    }
    if ($key -is [Security.Cryptography.RSACryptoServiceProvider]) {
        return -not $key.CspKeyContainerInfo.Exportable
    }
    return $false
} catch { return $false }
finally { if ($null -ne $key) { $key.Dispose() } }
