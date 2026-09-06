# Original user-profile environment behavior, with hidden input and sanitized failures.
param([string]$Address, [Security.SecureString]$Token)
$pointer = [IntPtr]::Zero
try {
    $pointer = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($Token)
    $value = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($pointer)
    if ([string]::IsNullOrWhiteSpace($value) -or $value -eq 'hvs.your-token') { throw 'TOKEN_EMPTY' }
    [Environment]::SetEnvironmentVariable('VAULT_ADDR', $Address, 'User')
    [Environment]::SetEnvironmentVariable('VAULT_TOKEN', $value, 'User')
    $env:VAULT_ADDR = $Address
    $env:VAULT_TOKEN = $value
    Write-Host 'Vault address and token saved for your Windows account. Token value is not displayed.'
} catch {
    # Never propagate a credential-bearing parameter or exception into transcripts.
    throw 'VAULT_CONFIG_SAVE_FAILED: A nonempty token and permission to save user environment settings are required. Token value was not logged.'
} finally {
    if ($pointer -ne [IntPtr]::Zero) { [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($pointer) }
    $value = $null
    $Token.Dispose()
}