//! Process-wide worker environment helpers.
//!
//! The worker exports control-plane details once during startup so spawned Git
//! helpers can authenticate against the server. The current implementation
//! uses `std::env::set_var` because external tools (git credential helper
//! scripts, commit-msg hooks) read these values from the process environment.
//!
//! # Thread Safety
//!
//! `std::env::set_var` is inherently unsafe in multi-threaded programs.
//! This function is called exactly once during worker startup, before the
//! Tokio runtime spawns any async tasks. A `OnceLock` guard ensures the
//! write cannot race with itself. Future refactors should pass these values
//! explicitly via `.env()` on each child `Command` instead.
//!
//! # Examples
//!
//! ```ignore
//! export_worker_runtime_env("https://api.codetether.run", &None, "wrk_1");
//! ```

use std::sync::OnceLock;

/// Guard ensuring `export_worker_runtime_env` runs at most once.
static EXPORTED: OnceLock<()> = OnceLock::new();

/// Exports worker runtime variables used by downstream Git helpers.
///
/// The worker writes these values exactly once near startup before child
/// processes inherit the environment. Panics if called more than once.
///
/// # Examples
///
/// ```ignore
/// export_worker_runtime_env("https://api.codetether.run", &None, "wrk_1");
/// ```
pub(super) fn export_worker_runtime_env(
    server: &str,
    token: &Option<String>,
    worker_id: &str,
) {
    EXPORTED.get_or_init(|| {
        // SAFETY: Called exactly once at startup, before the Tokio runtime
        // spawns any async tasks. The `OnceLock` guarantees single-execution.
        // Git credential helper scripts and commit-msg hooks require these
        // values in the process environment. Future refactors should migrate
        // to explicit `.env()` injection on each `Command` and remove this
        // `set_var` call entirely.
        unsafe {
            std::env::set_var("CODETETHER_SERVER", server);
            std::env::set_var("CODETETHER_WORKER_ID", worker_id);
            if let Some(token) = token {
                std::env::set_var("CODETETHER_TOKEN", token);
            }
        }
    });
}
