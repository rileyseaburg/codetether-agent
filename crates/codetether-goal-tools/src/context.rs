//! Trusted session identity extraction for goal tool calls.

use anyhow::{Result, anyhow};

/// Resolve runtime session context, falling back to the existing environment key.
/// # Arguments
/// * `injected` - Session identifier supplied by the runtime.
/// # Returns
/// The injected identifier or environment fallback.
/// # Errors
/// Returns an error when neither source provides an identifier.
/// # Examples
/// ```
/// assert_eq!(codetether_goal_tools::context::session_id(Some("s".into())).unwrap(), "s");
/// ```
pub fn session_id(injected: Option<String>) -> Result<String> {
    injected
        .or_else(|| std::env::var("CODETETHER_SESSION_ID").ok())
        .ok_or_else(|| anyhow!("session id not available; cannot locate goal"))
}
