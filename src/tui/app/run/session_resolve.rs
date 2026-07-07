//! Session resolution at TUI startup.
//!
//! Replaces the old "create blank first, overwrite later" pattern with a
//! single decision point:
//!
//! 1. Prior session found → use it directly (no blank allocation).
//! 2. No prior session (fresh workspace) → create new silently.
//! 3. Scan failed (timeout, corrupt file, etc.) → create new AND push a
//!    visible warning into the chat so the user knows what happened.

use std::sync::Arc;

use anyhow::Result;

use crate::bus::AgentBus;
use crate::session::{Session, TailLoad};

/// Resolve the startup session from the scan result.
///
/// - `Ok(tail)` → load the prior session; do **not** fork on truncation at
///   startup (the ID is preserved; the status bar will show how many messages
///   were dropped).
/// - `Err(reason)` where the reason is the expected "no prior session" case
///   → create new silently.
/// - `Err(reason)` that looks like a real failure (timeout, parse error) →
///   create new but record the warning so `hydrate::complete` can surface it.
pub(super) async fn resolve(
    scan: Option<Result<TailLoad>>,
    bus: &Arc<AgentBus>,
) -> Result<Session> {
    match scan.unwrap_or_else(|| Err(anyhow::anyhow!("no scan result"))) {
        Ok(load) => {
            // Good load — use it directly, no blank session ever allocated.
            let session = load.session.with_bus(bus.clone());
            if load.dropped > 0 {
                tracing::warn!(
                    session_id = %session.id,
                    dropped = load.dropped,
                    file_bytes = load.file_bytes,
                    "session resumed with tail-cap: oldest {} messages not loaded",
                    load.dropped,
                );
            }
            Ok(session)
        }
        Err(err) => {
            let reason = err.to_string();
            // Distinguish expected "nothing to resume" from real failures.
            if is_expected_new_session(&reason) {
                tracing::debug!("no prior session for workspace — starting fresh");
            } else {
                // Real failure: timeout, corrupt JSON, etc.
                // Store in process env so hydrate::complete can surface it.
                tracing::warn!(reason = %reason, "session scan failed — starting fresh session");
                // SAFETY: single-threaded startup, env var is read once in hydrate.
                unsafe {
                    std::env::set_var("CODETETHER_SESSION_LOAD_WARNING", &reason);
                }
            }
            Ok(Session::new().await?.with_bus(bus.clone()))
        }
    }
}

/// Returns true when the error is the normal "nothing to resume" case rather
/// than a real failure the user should know about.
fn is_expected_new_session(reason: &str) -> bool {
    reason.contains("requested fresh session")
        || reason.contains("no session")
        || reason.contains("No such file")
        || reason.contains("not found")
}
