# Validate access before replacing stored credentials; never emit secret responses.
param([string]$Address, [Security.SecureString]$Token, [string]$Mount = 'secret', [string]$ProviderPath = 'codetether/providers')
$ErrorActionPreference = 'Stop'
$uri = $null
if (-not [Uri]::TryCreate($Address, [UriKind]::Absolute, [ref]$uri) -or $uri.UserInfo -or $uri.Query -or $uri.Fragment -or ($uri.Scheme -ne 'https' -and -not ($uri.Scheme -eq 'http' -and $uri.IsLoopback))) {
    throw 'VAULT_ADDR_INVALID: Use HTTPS without embedded credentials (HTTP is allowed only on loopback).'
}
$pointer = [IntPtr]::Zero
$headers = @{}
try {
    $pointer = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($Token)
    $value = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($pointer)
    if ([string]::IsNullOrWhiteSpace($value)) { throw 'EMPTY_TOKEN' }
    $headers['X-Vault-Token'] = $value
    $base = $Address.TrimEnd('/')
    $lookup = $null
    try { $lookup = Invoke-RestMethod "$base/v1/auth/token/lookup-self" -Headers $headers -TimeoutSec 20 -MaximumRedirection 0 }
    catch { $lookup = $null }
    $nonexpiring = $null -ne $lookup -and $null -ne $lookup.data -and $null -ne $lookup.data.ttl -and $lookup.data.ttl -eq 0
    if ($lookup -and $lookup.data.num_uses -gt 0) { throw 'USE_LIMITED_TOKEN_UNSUPPORTED' }
    if (-not $nonexpiring) {
        $renewed = Invoke-RestMethod "$base/v1/auth/token/renew-self" -Method Post -Headers $headers -ContentType 'application/json' -Body '{}' -TimeoutSec 20 -MaximumRedirection 0
        if ($renewed.auth.renewable -ne $true -or $renewed.auth.lease_duration -le 0) { throw 'RENEWABLE_TOKEN_REQUIRED' }
    }
    $mountPath = ($Mount.Trim('/') -split '/' | ForEach-Object { [Uri]::EscapeDataString($_) }) -join '/'
    $providerPathEncoded = ($ProviderPath.Trim('/') -split '/' | ForEach-Object { [Uri]::EscapeDataString($_) }) -join '/'
    $providers = Invoke-RestMethod "$base/v1/$mountPath/metadata/${providerPathEncoded}?list=true" -Headers $headers -TimeoutSec 20 -MaximumRedirection 0
    if ($null -eq $providers.data.keys -or @($providers.data.keys).Count -eq 0) { throw 'PROVIDER_LIST_EMPTY' }
    $readable = $false
    foreach ($name in @($providers.data.keys)) {
        if ($name.EndsWith('/')) { continue }
        $secretName = [Uri]::EscapeDataString($name)
        try { $null = Invoke-RestMethod "$base/v1/$mountPath/data/$providerPathEncoded/$secretName" -Headers $headers -TimeoutSec 20 -MaximumRedirection 0; $readable = $true; break }
        catch { continue }
    }
    if (-not $readable) { throw 'PROVIDER_READ_DENIED' }
    [pscustomobject]@{ Valid = $true; ProviderCount = @($providers.data.keys).Count }
} catch {
    # Do not include exception bodies, headers, parameter values or token data.
    throw 'VAULT_VALIDATION_FAILED: Token lookup/renewal or provider listing failed. Obtain a valid renewable token with access to the configured provider path. Saved credentials were not changed.'
} finally {
    if ($pointer -ne [IntPtr]::Zero) { [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($pointer) }
    $headers.Clear(); $value = $null; $lookup = $null; $renewed = $null; $providers = $null
}