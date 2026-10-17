//! Immediate-child fallback for platforms without process-tree primitives.

use tokio::process::Command;

/// No-op descendant guard; `kill_on_drop` still owns the immediate child.
pub(super) struct Guard;

impl Guard {
    /// Creates the fallback guard without a platform process identifier.
    pub(super) fn attach(_pid: Option<u32>) -> Self {
        Self
    }

    /// Completes fallback tracking after normal process exit.
    pub(super) fn disarm(&mut self) {}

    /// Relies on the Tokio child owner's `kill_on_drop` behavior.
    pub(super) fn terminate(&mut self) {}
}

/// Requires no platform-specific command setup.
pub(super) fn configure(_command: &mut Command) {}
