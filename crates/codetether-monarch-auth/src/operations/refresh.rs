//! Noninteractive recovery using only the previously authorized OAuth grant.
use anyhow::Result;

/// Refresh the saved Monarch grant without browser consent or token output.
///
/// # Arguments
/// None; uses the configured secret backend and existing read-only grant.
/// # Returns
/// Unit after refreshed credentials have been persisted. This does not prove
/// MCP access or retry any financial operation.
/// # Errors
/// Returns sanitized errors if the grant is absent, cannot refresh, has broader
/// scopes, or cannot be persisted. Never requests additional authorization.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_monarch_auth::refresh().await
/// # }
/// ```
pub async fn refresh() -> Result<()> {
    super::access::acquire(true).await.map(|_| ())
}
