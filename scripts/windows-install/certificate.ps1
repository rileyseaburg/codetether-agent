# The private key remains non-exportable in the installing user's personal store.
$sid = [Security.Principal.WindowsIdentity]::GetCurrent().User.Value
$subject = "CN=CodeTether Local $sid"
$cert = Get-ChildItem Cert:\CurrentUser\My -CodeSigningCert | Where-Object {
    $_.Subject -eq $subject -and $_.HasPrivateKey -and $_.NotAfter -gt (Get-Date).AddDays(30) -and
    (& "$PSScriptRoot\key-policy.ps1" -Certificate $_)
} | Sort-Object NotAfter -Descending | Select-Object -First 1
if (-not $cert) {
    $cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject $subject `
        -CertStoreLocation Cert:\CurrentUser\My -KeyExportPolicy NonExportable `
        -Provider 'Microsoft Software Key Storage Provider' `
        -KeyAlgorithm RSA -KeyLength 3072 -HashAlgorithm SHA256 -KeyUsage DigitalSignature `
        -TextExtension @('2.5.29.19={critical}{text}ca=false') -NotAfter (Get-Date).AddYears(2)
}
if (-not (& "$PSScriptRoot\key-policy.ps1" -Certificate $cert)) { throw 'SIGNING_KEY_POLICY_REJECTED: Private-key export must be disabled.' }
$cert
