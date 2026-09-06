# Inline elevated operation; accepts public DER bytes, never a file path/private key.
param([string]$Certificate)
if (-not $Certificate) { return }
$cert = New-Object Security.Cryptography.X509Certificates.X509Certificate2(,[Convert]::FromBase64String($Certificate))
$constraints = @($cert.Extensions | Where-Object { $_.Oid.Value -eq '2.5.29.19' })
$eku = @($cert.Extensions | Where-Object { $_.Oid.Value -eq '2.5.29.37' })
if ($constraints.Count -ne 1 -or $constraints[0].CertificateAuthority -or
    $eku.Count -ne 1 -or @($eku[0].EnhancedKeyUsages | Where-Object { $_.Value -eq '1.3.6.1.5.5.7.3.3' }).Count -ne 1 -or
    $cert.Subject -ne $cert.Issuer -or $cert.Subject -notmatch '^CN=CodeTether Local S-[0-9-]+$') {
    throw 'CERTIFICATE_REJECTED: Only the generated self-signed, non-CA code-signing leaf may be trusted.'
}
$store = New-Object Security.Cryptography.X509Certificates.X509Store('TrustedPeople', 'LocalMachine')
try {
    $store.Open([Security.Cryptography.X509Certificates.OpenFlags]::ReadWrite)
    $store.Add($cert)
    Write-Host "Trusted leaf $($cert.Thumbprint) in LocalMachine/TrustedPeople (not Root)."
} finally { $store.Close(); $cert.Dispose() }
