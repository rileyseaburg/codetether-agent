//! Process-wide worker environment helpers.
//!
//! The worker exports control-plane details once during startup so spawned Git
//! helpers can authenticate against the server without bespoke wiring.
//!
//! # Examples
//!
//! ```ignore
//! export_worker_runtime_env("https://api.codetether.run", &None, "wrk_1");
//! ```

/// Exports worker runtime variables used by downstream Git helpers.
///
/// The worker writes these values exactly once near startup before child
/// processes inherit the environment.
///
/// # Examples
///
/// ```ignore
/// export_worker_runtime_env("https://api.codetether.run", &None, "wrk_1");
/// ```
pub(super) fn export_worker_runtime_env(server: &str, token: &Option<String>, worker_id: &str) {
    // SAFETY: The worker sets these process-wide variables once during startup before
    // spawning Git helper child processes. They are required so Git credential helpers
    // invoked by later shell/git commands can reach the control plane securely.
    unsafe {
        std::env::set_var("CODETETHER_SERVER", server);
        std::env::set_var("CODETETHER_WORKER_ID", worker_id);
        if let Some(token) = token {
            std::env::set_var("CODETETHER_TOKEN", token);
        }
    }
}
