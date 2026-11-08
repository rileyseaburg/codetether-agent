//! Non-disclosing inspection of the configured credential record.
use crate::storage::Store;
use anyhow::Result;

/// Report credential-record presence without contacting Monarch.
/// # Arguments
/// None; uses the application's configured secret backend and namespace.
/// # Returns
/// Whether a valid credential record exists, not whether its grant remains valid.
/// # Errors
/// Returns backend availability, authorization or malformed-record errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let present = codetether_monarch_auth::status().await?;
/// # let _ = present;
/// # Ok(()) }
/// ```
pub async fn status() -> Result<bool> {
    Ok(Store::open()?.load().await?.is_some())
}
