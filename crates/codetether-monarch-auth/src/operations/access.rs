//! Acquire a read-only token and persist any refresh rotation before use.
use crate::{discovery, oauth, scope, storage::Store};
use anyhow::{Result, bail};

/// Obtain a bearer token from the configured backend, refreshing if necessary.
/// # Arguments
/// None; the application selects the secret backend and namespace.
/// # Returns
/// A secret for the pinned Monarch HTTP Authorization header only.
/// Never include the returned token in logs, tool output or model context.
/// # Errors
/// Returns sanitized backend, missing-grant, scope or refresh errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let token = codetether_monarch_auth::access_token().await?;
/// assert!(!token.is_empty());
/// # Ok(()) }
/// ```
pub async fn access_token() -> Result<String> {
    acquire(false).await
}

pub(super) async fn acquire(force_refresh: bool) -> Result<String> {
    let store = Store::open()?;
    let _lock = store.lock().await?;
    let Some(mut credentials) = store.load().await? else {
        bail!("Monarch is not connected; run codetether auth monarch");
    };
    scope::require_read_only(&credentials.scope)?;
    if force_refresh || !credentials.fresh()? {
        let client = discovery::client()?;
        discovery::discover(&client).await?;
        let rotated = oauth::refresh(&client, &credentials).await?;
        store.save_rotated(&credentials, &rotated).await?;
        credentials = rotated;
    }
    Ok(credentials.access_token)
}
