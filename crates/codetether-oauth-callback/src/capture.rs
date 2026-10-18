//! OAuth callback capture handling.
use crate::{
    CODEX_CALLBACK_ADDR_V4, CODEX_CALLBACK_ADDR_V6, CODEX_CALLBACK_DISPLAY_ADDR,
    wait_any::wait_for_oauth_callback_any,
};
use anyhow::Result;
use tokio::time::Duration;

/// Try automatic capture on the existing Codex IPv4 and IPv6 callback addresses.
///
/// # Arguments
/// timeout is the maximum duration supplied to each listener.
/// # Returns
/// The captured code/state pair, or None when binding or capture fails.
/// # Errors
/// Capture errors are logged and converted to None to permit manual paste.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let callback = codetether_oauth_callback::capture_oauth_callback_auto(std::time::Duration::from_secs(30)).await?;
/// # Ok(()) }
/// ```
pub async fn capture_oauth_callback_auto(timeout: Duration) -> Result<Option<(String, String)>> {
    let listeners = super::bind::listeners().await;

    if listeners.is_empty() {
        tracing::warn!(
            ipv4 = CODEX_CALLBACK_ADDR_V4,
            ipv6 = CODEX_CALLBACK_ADDR_V6,
            "Failed to bind OAuth callback listener on localhost addresses; falling back to manual paste"
        );
        return Ok(None);
    }

    println!(
        "Waiting up to {}s for automatic callback capture on http://{}/auth/callback ...",
        timeout.as_secs(),
        CODEX_CALLBACK_DISPLAY_ADDR
    );

    match wait_for_oauth_callback_any(listeners, timeout).await {
        Ok(callback) => Ok(Some(callback)),
        Err(error) => {
            tracing::warn!(
                error = %error,
                "Automatic OAuth callback capture did not complete; falling back to manual paste"
            );
            Ok(None)
        }
    }
}
