//! Prefer explicit calling-session identity before directory fallback.
use crate::errors::classify;
use crate::{SESSION_ID_FIELD, Source};
use anyhow::Result;
use serde_json::Value;

/// Load the session that issued this tool call.
///
/// Returns `Ok(None)` when no session exists, `Ok(Some(session))`
/// on success, or `Err` on I/O/parse failures.
///
/// # Arguments
/// * `args` - Tool arguments containing optional injected session identity.
/// # Returns
/// The calling session, if found, with an unbounded message window.
/// # Errors
/// Returns storage errors other than recognized missing-session errors.
/// # Examples
/// ```rust,no_run
/// # async fn example<S:codetether_context_loader::Source>()->anyhow::Result<()> {
/// let _session=codetether_context_loader::load_calling_session::<S>(&serde_json::json!({"__ct_session_id":"session"})).await?;
/// # Ok(()) }
/// ```
pub async fn load_calling_session<S: Source>(args: &Value) -> Result<Option<S>> {
    load_calling_window::<S>(args, usize::MAX).await
}

/// Load only a bounded working window of the calling session.
///
/// # Arguments
/// * `args` - Tool arguments containing optional injected session identity.
/// * `window` - Maximum message window requested from storage.
/// # Returns
/// The calling session, or None if the selected session does not exist.
/// # Errors
/// Returns storage errors other than recognized missing-session errors.
/// # Examples
/// ```rust,no_run
/// # async fn example<S:codetether_context_loader::Source>()->anyhow::Result<()> {
/// let _session=codetether_context_loader::load_calling_window::<S>(&serde_json::json!({}),50).await?;
/// # Ok(()) }
/// ```
pub async fn load_calling_window<S: Source>(args: &Value, window: usize) -> Result<Option<S>> {
    let injected = args
        .get(SESSION_ID_FIELD)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(String::from)
        .or_else(|| std::env::var("CODETETHER_SESSION_ID").ok());
    match injected {
        Some(id) => classify(S::load_tail(&id, window).await),
        None => {
            let cwd = std::env::current_dir().ok();
            classify(S::last_for_directory_tail(cwd.as_deref(), window).await)
        }
    }
}
