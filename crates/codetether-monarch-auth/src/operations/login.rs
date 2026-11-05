//! Interactive OAuth authorization and configured-backend persistence.
use crate::{discovery, oauth, storage::Store};
use anyhow::Result;

/// Authorize Monarch using a loopback callback and PKCE S256.
/// # Arguments
/// * `write` - Request write scope only when explicitly selected by an embedding client.
///   The built-in CLI always supplies false.
/// # Returns
/// Unit after the configured secret backend accepts the credentials.
/// # Errors
/// Returns sanitized backend, consent, discovery, exchange or timeout errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_monarch_auth::login(false).await?;
/// # Ok(()) }
/// ```
pub async fn login(write: bool) -> Result<()> {
    let store = Store::open()?;
    store.preflight().await?;
    let _lock = store.lock().await?;
    let client = discovery::client()?;
    discovery::discover(&client).await?;
    let credentials = oauth::authorize(&client, write).await?;
    store.save(&credentials).await
}
