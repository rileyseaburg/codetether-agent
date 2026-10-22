//! Resolve user-message evidence through the native session locator.
/// Find the latest canonical user source without changing session state.
/// # Arguments
/// * `id` — Durable native session identity.
/// # Returns
/// Message identity, text and digest, or none for missing indexed evidence.
/// # Errors
/// Returns identity, path, storage, decoding or worker errors.
/// # Panics
/// Panics when called without a running Tokio runtime.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _source = codetether_native_goals::source::latest("session-123").await?;
/// # Ok(()) }
/// ```
pub async fn latest(
    id: &str,
) -> anyhow::Result<Option<codetether_session_store::user_source::UserSource>> {
    let path = codetether_session_store::lifecycle::session_path(id)?;
    let id = id.to_owned();
    tokio::task::spawn_blocking(move || codetether_session_store::user_source::read(&path, &id))
        .await?
}
