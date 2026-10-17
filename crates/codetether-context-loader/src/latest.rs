//! Explicit latest-session lookup for callers without a durable identity.
use crate::{Source, errors::classify};
use anyhow::Result;

/// Load the latest session for the current working directory.
///
/// Prefer [`crate::load_calling_session`]; this cannot distinguish concurrent
/// sessions rooted in the same checkout.
///
/// # Arguments
/// None; uses the current process directory when available.
/// # Returns
/// The newest session selected by the storage backend, or None when missing.
/// # Errors
/// Returns storage errors other than recognized missing-session errors.
/// # Examples
/// ```rust,no_run
/// # async fn example<S:codetether_context_loader::Source>()->anyhow::Result<()> {
/// let _session=codetether_context_loader::load_latest_session::<S>().await?;
/// # Ok(()) }
/// ```
pub async fn load_latest_session<S: Source>() -> Result<Option<S>> {
    let cwd = std::env::current_dir().ok();
    classify(S::last_for_directory(cwd.as_deref()).await)
}
