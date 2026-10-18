use anyhow::{Result, bail};

/// Wait until no live server hosts `name`.
/// # Arguments
/// * `name` - Session whose registry disappearance is awaited.
/// # Returns
/// Unit when no server record contains the session.
/// # Errors
/// Returns registry errors or timeout after the standard backoff budget.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_lifecycle::restart::wait_stopped("work").await?;
/// # Ok(()) }
/// ```
pub async fn wait_stopped(name: &str) -> Result<()> {
    let mut backoff = crate::backoff::Backoff::new();
    while backoff.remaining() {
        if codetether_mux_registry::find_session(name).await?.is_none() {
            return Ok(());
        }
        backoff.sleep().await;
    }
    bail!("mux session did not stop cleanly")
}
