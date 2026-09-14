param([string]$Root, [string]$Work)
$global:recoveryCase = 'valid'
$global:renewCalled = 0
$global:recoverySecret = 'fixture-' + [guid]::NewGuid()
function Invoke-RestMethod {
    param($Uri, $Headers, $Method, $ContentType, $Body, $TimeoutSec, $MaximumRedirection)
    Assert-Contract ($MaximumRedirection -eq 0) 'Vault token must not follow redirects'
    Assert-Contract ($Headers['X-Vault-Token'] -eq $global:recoverySecret) 'candidate token is used only in the header'
    if ($global:recoveryCase -eq 'denied') { throw 'fixture rejection' }
    if ($Uri -match 'lookup-self') {
        if ($global:recoveryCase -eq 'lookup-denied') { throw 'lookup forbidden' }
        return [pscustomobject]@{ data = [pscustomobject]@{ ttl = 60; num_uses = 0 } }
    }
    if ($Uri -match 'renew-self') {
        $global:renewCalled++
        return [pscustomobject]@{ auth = [pscustomobject]@{ renewable = ($global:recoveryCase -ne 'nonrenewable'); lease_duration = 60 } }
    }
    if ($Uri -match '/data/') {
        if ($global:recoveryCase -eq 'read-denied') { throw 'provider read forbidden' }
        return [pscustomobject]@{ data = [pscustomobject]@{ data = @{} } }
    }
    $keys = if ($global:recoveryCase -eq 'empty') { @() } else { @('configured-provider') }
    [pscustomobject]@{ data = [pscustomobject]@{ keys = $keys } }
}
$token = ConvertTo-SecureString $global:recoverySecret -AsPlainText -Force
try {
    foreach ($case in @('valid', 'lookup-denied', 'denied', 'nonrenewable', 'empty', 'read-denied')) {
        $global:recoveryCase = $case
        $action = { & "$Root/verify-vault-token.ps1" -Address 'https://vault.example.invalid' -Token $token }
        if ($case -in @('valid','lookup-denied')) {
            $result = & $action
            Assert-Contract ($result.Valid -and $result.ProviderCount -eq 1) 'candidate access verified'
            Assert-Contract (-not ($result | ConvertTo-Json).Contains($global:recoverySecret)) 'no token in verification output'
        } else { Assert-Throws $action 'VAULT_VALIDATION_FAILED' }
    }
    Assert-Contract ($global:renewCalled -ge 2) 'renewability is tested, not guessed'
} finally { $token.Dispose() }
$text = Get-Content "$Root/repair-vault.ps1" -Raw
Assert-Contract ($text.IndexOf('verify-vault-token.ps1') -lt $text.IndexOf('save-vault.ps1')) 'validation precedes persisted replacement'
Assert-Contract ($text.Contains('reloads its caller explicitly')) 'does not promise terminal restarts refresh credentials'
Assert-Contract ($text.IndexOf('VAULT_ADDR_INVALID') -lt $text.IndexOf('vault-token.ps1')) 'address checked before token prompt'