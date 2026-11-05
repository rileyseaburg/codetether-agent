//! Revoke the grant and clear only its configured-backend record fields.
use crate::{oauth, storage::Store};
use anyhow::Result;

/// Revoke Monarch access and clear the configured credential record.
/// # Arguments
/// None; uses the application's configured backend and namespace.
/// # Returns
/// Unit after revocation and clearing, or when no record exists.
/// The record is cleared even if remote revocation fails.
/// # Errors
/// Returns backend or remote-revocation errors. Vault history/backups may retain
/// prior versions; clearing fields is not forensic erasure.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_monarch_auth::logout().await?;
/// # Ok(()) }
/// ```
pub async fn logout() -> Result<()> {
    let store = Store::open()?;
    let _lock = store.lock().await?;
    let result = match store.load().await {
        Ok(Some(credentials)) => oauth::revoke(&credentials).await,
        Ok(None) => Ok(()),
        Err(error) => Err(error),
    };
    store.erase().await?;
    result
}
