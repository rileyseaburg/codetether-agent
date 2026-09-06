param([string]$Root, [string]$Work)
# Construct rejected certificates in memory; never touch a certificate store.
$rsa = [Security.Cryptography.RSA]::Create(2048)
try {
    $request = [Security.Cryptography.X509Certificates.CertificateRequest]::new(
        'CN=CodeTether Local S-1-5-21-123', $rsa, [Security.Cryptography.HashAlgorithmName]::SHA256,
        [Security.Cryptography.RSASignaturePadding]::Pkcs1)
    $request.CertificateExtensions.Add([Security.Cryptography.X509Certificates.X509BasicConstraintsExtension]::new($true, $false, 0, $true))
    $usages = [Security.Cryptography.OidCollection]::new()
    [void]$usages.Add([Security.Cryptography.Oid]::new('1.3.6.1.5.5.7.3.3'))
    $request.CertificateExtensions.Add([Security.Cryptography.X509Certificates.X509EnhancedKeyUsageExtension]::new($usages, $false))
    $cert = $request.CreateSelfSigned([DateTimeOffset]::UtcNow.AddMinutes(-1), [DateTimeOffset]::UtcNow.AddDays(1))
    try {
        $public = [Convert]::ToBase64String($cert.RawData)
        Assert-Throws { & "$Root/trust-leaf.ps1" -Certificate $public } 'CERTIFICATE_REJECTED'
    } finally { $cert.Dispose() }
} finally { $rsa.Dispose() }
$code = Get-Content "$Root/trust-leaf.ps1" -Raw
Assert-Contract ($code -match "X509Store\('TrustedPeople', 'LocalMachine'\)") 'leaf trust store fixed in code'
Assert-Contract ($code -notmatch "X509Store\('Root'") 'root store never opened'
