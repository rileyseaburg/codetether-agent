//! Resolve one named mux session from the authenticated local registry.

use anyhow::Result;

use codetether_mux_registry::SessionTarget;

/// Find the server hosting session `name`, if any record lists it.
///
/// # Errors
///
/// Returns an error when the mux registry cannot be scanned.
pub(super) async fn load(name: &str) -> Result<Option<SessionTarget>> {
    codetether_mux_registry::find_session(name).await
}
