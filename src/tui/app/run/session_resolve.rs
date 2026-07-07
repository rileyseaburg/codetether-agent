//! Session resolution at TUI startup.
//!
//! Replaces the old "create blank first, overwrite later" pattern with a
//! single decision point that returns both the resolved session **and** a
//! description of how it was resolved — no env-var side-channel, no
//! after-the-fact heuristic classification.
//!
//! 1. Prior session found → use it directly (no blank allocation).
//! 2. No prior session (fresh workspace) → create new silently.
//! 3. Scan failed (timeout, corrupt file, etc.) → create new AND return a
//!    [`SessionLoadOutcome::ScanFailed`] so the caller can warn the user.

use std::sync::Arc;

use anyhow::Result;

use crate::bus::AgentBus;
use crate::session::{Session, TailLoad};

use super::session_outcome::SessionLoadOutcome;
use super::session_resolve_helpers::{resolve_loaded, resolve_new};

/// A resolved startup session paired with how it was obtained.
pub(super) struct Resolved {
    pub session: Session,
    pub outcome: SessionLoadOutcome,
}

/// Resolve the startup session from the scan result.
///
/// - `Ok(tail)` → load the prior session directly. The ID is preserved; the
///   `dropped` count (tail-cap truncation) is reported in the outcome.
/// - `Err(reason)` for the expected "no prior session" case → create new,
///   report [`SessionLoadOutcome::Fresh`].
/// - `Err(reason)` for a real failure (timeout, parse error) → create new,
///   report [`SessionLoadOutcome::ScanFailed`] so the user is warned.
pub(super) async fn resolve(
    scan: Option<Result<TailLoad>>,
    bus: &Arc<AgentBus>,
) -> Result<Resolved> {
    match scan.unwrap_or_else(|| Err(anyhow::anyhow!("no scan result"))) {
        Ok(load) => Ok(resolve_loaded(load, bus)),
        Err(err) => resolve_new(err.to_string(), bus).await,
    }
}
