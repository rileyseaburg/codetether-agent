//! Process-wide shared reference to the active [`AgentBus`].
//!
//! Spawned sub-agent sessions (created via the `spawn` tool) need to publish
//! on the same bus as their parent, but the tool trait's `execute` method
//! takes only a [`serde_json::Value`] — there is no way to thread the parent
//! bus through the call chain without changing every tool.
//!
//! Entrypoints (CLI `run`, `serve`, `tui`, `ralph`) call [`set_global`] once
//! after constructing their bus; factories that build child sessions call
//! [`global`] to attach the same bus.
//!
//! The global is set **at most once** per process. Subsequent calls to
//! [`set_global`] are no-ops.

use std::sync::{Arc, OnceLock};

use super::AgentBus;

static GLOBAL_BUS: OnceLock<Arc<AgentBus>> = OnceLock::new();

/// Install the process-wide bus. Idempotent — first writer wins.
///
/// # Arguments
/// * `bus` - Shared bus used by application entrypoints and tools.
///
/// # Returns
/// Returns without replacing a bus that is already installed.
///
/// # Examples
/// ```rust,no_run
/// # fn install(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
/// codetether_bus_core::global::set_global(bus);
/// # }
/// ```
pub fn set_global(bus: Arc<AgentBus>) {
    let _ = GLOBAL_BUS.set(bus);
}

/// Return a clone of the process-wide bus, if one has been installed.
///
/// # Arguments
/// Takes no arguments.
///
/// # Returns
/// The installed shared bus, or `None` before installation.
///
/// # Examples
/// ```rust
/// let bus = codetether_bus_core::global::global();
/// assert!(bus.is_none());
/// ```
pub fn global() -> Option<Arc<AgentBus>> {
    GLOBAL_BUS.get().cloned()
}

#[cfg(test)]
mod tests;
