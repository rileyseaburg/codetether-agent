//! Detached snapshot construction and transcript window selection.
use super::Session;
use codetether_provider_core::Message;
/// Build and persist a fresh session containing the configured source tail.
/// # Arguments
/// Receives the source and a lazily evaluated window size after initialization.
/// # Returns
/// A saved child with copied metadata, classified pages and a detached label.
/// # Errors
/// Propagates session initialization and persistence errors.
/// # Examples
/// ```no_run
/// async fn snapshot<S: codetether_session_fork::detached::Session>(parent: &S) -> anyhow::Result<S> {
///     codetether_session_fork::detached::create(parent, || 100).await
/// }
/// ```
pub async fn create<S: Session>(parent: &S, window: impl FnOnce() -> usize) -> anyhow::Result<S> {
    let mut child = S::new().await?;
    child.replace_transcript(tail(parent, window()));
    child.set_metadata(parent.metadata().clone());
    child.set_detached_agent();
    child.set_title(
        parent
            .title()
            .map(|t| format!("{t} (detached)"))
            .or_else(|| Some("detached".to_string())),
    );
    child.attach_bus_if_missing();
    child.save().await?;
    Ok(child)
}
/// Clone at most the requested number of most recent working messages.
/// # Arguments
/// Receives the parent session and maximum message count.
/// # Returns
/// A chronological tail, empty when the requested window is zero.
/// # Examples
/// ```
/// fn bounded<S: codetether_session_fork::detached::Session>(parent: &S) {
///     assert!(codetether_session_fork::detached::tail(parent, 2).len() <= 2);
/// }
/// ```
pub fn tail<S: Session>(parent: &S, window: usize) -> Vec<Message> {
    let start = parent.messages().len().saturating_sub(window);
    parent.messages()[start..].to_vec()
}
