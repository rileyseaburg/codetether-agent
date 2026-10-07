//! Transient-error retry helpers for Vault secret reads.
//!
//! Vault is frequently fronted by a reverse proxy or CDN (e.g. Cloudflare)
//! that returns transient `502/503/504` gateway errors. A single such blip
//! must not be mistaken for "secret missing", so reads are retried with a
//! short backoff before surfacing the error to callers.

use vaultrs::client::VaultClient;
use vaultrs::error::ClientError;
use vaultrs::kv2;

use super::ProviderSecrets;

/// Whether an error is a transient gateway failure worth retrying
/// (e.g. Cloudflare 502/503/504 in front of Vault).
pub(super) fn is_transient_gateway_error(err: &ClientError) -> bool {
    matches!(
        err,
        ClientError::APIError { code, .. } if matches!(code, 502 | 503 | 504)
    )
}

/// Read provider secrets, retrying transient 5xx gateway errors.
pub(super) async fn read_secret_with_retry(
    client: &VaultClient,
    mount: &str,
    secret_path: &str,
) -> std::result::Result<ProviderSecrets, ClientError> {
    let mut attempt = 0u32;
    loop {
        match kv2::read::<ProviderSecrets>(client, mount, secret_path).await {
            Err(err) if is_transient_gateway_error(&err) && attempt < 3 => {
                attempt += 1;
                tracing::warn!(attempt, "Transient Vault gateway error, retrying: {err}");
                let delay = std::time::Duration::from_millis(250 * u64::from(attempt));
                tokio::time::sleep(delay).await;
            }
            other => return other,
        }
    }
}
