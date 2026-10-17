//! Upgrade an unsandboxed selection to macOS Seatbelt when it is available.

use super::Runner;
use std::path::PathBuf;

/// Replace a `Direct` selection with Seatbelt when `sandbox-exec` is present.
///
/// Bubblewrap selections are left untouched so Linux keeps its stronger
/// namespace isolation.
pub(super) fn upgrade(runner: Runner, sandbox_exec: Option<PathBuf>) -> Runner {
    match (runner, sandbox_exec) {
        (Runner::Direct(_), Some(path)) => Runner::Seatbelt(path),
        (runner, _) => runner,
    }
}

#[cfg(test)]
#[path = "sandbox_runner_select_seatbelt_tests.rs"]
mod tests;
