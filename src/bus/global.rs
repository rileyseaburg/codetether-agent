//! Process-wide shared reference to the active [`AgentBus`].
//!
//! Spawned sub-agent sessions (created via the `spawn` tool) need to publish
//! on the same bus as their parent, but the tool trait's [`execute`] method
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
pub fn set_global(bus: Arc<AgentBus>) {
    let _ = GLOBAL_BUS.set(bus);
}

/// Return a clone of the process-wide bus, if one has been installed.
pub fn global() -> Option<Arc<AgentBus>> {
    GLOBAL_BUS.get().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_is_none_by_default_or_already_set() {
        // Cannot reliably test set_global in a unit test because OnceLock
        // persists across tests in the same binary. We only assert that
        // `global()` either returns None or returns Some consistently.
        let first = global().map(|b| Arc::as_ptr(&b) as usize);
        let second = global().map(|b| Arc::as_ptr(&b) as usize);
        assert_eq!(first, second);
    }
}
