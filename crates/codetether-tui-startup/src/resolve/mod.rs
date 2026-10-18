//! Startup scanning and fresh/loaded session resolution.
//!
//! [scan] loads only explicitly requested sessions. [resolve] attaches the bus
//! and records a fresh/loaded/failed-scan outcome, preserving truncation forks.
mod helpers;
mod host;
mod scan;
use crate::Resolved;
use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_session_store::TailLoad;
use helpers::{resolve_loaded, resolve_new};
pub use host::Host;
pub use scan::scan;
use std::sync::Arc;
/// Resolve a loaded or fresh session and retain the startup outcome.
/// # Arguments
/// Receives an optional scan result and the process bus to attach.
/// # Returns
/// The session and explicit resolution outcome.
/// # Errors
/// Returns native session creation or truncated-history continuation errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_tui_startup::resolve::Host>(bus: &std::sync::Arc<codetether_bus_core::AgentBus>) {
/// let _ = codetether_tui_startup::resolve::resolve::<H>(None, bus).await;
/// # }
/// ```
pub async fn resolve<H: Host>(
    scan: Option<Result<TailLoad<H::Session>>>,
    bus: &Arc<AgentBus>,
) -> Result<Resolved<H::Session>> {
    match scan.unwrap_or_else(|| Err(anyhow::anyhow!("no scan result"))) {
        Ok(load) => resolve_loaded::<H>(load, bus).await,
        Err(err) => resolve_new::<H>(err.to_string(), bus).await,
    }
}
