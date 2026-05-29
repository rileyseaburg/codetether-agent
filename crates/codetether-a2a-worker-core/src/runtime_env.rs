//! Process-wide worker environment helpers.

use std::sync::OnceLock;

static EXPORTED: OnceLock<()> = OnceLock::new();

pub fn export_worker_runtime_env(server: &str, token: &Option<String>, worker_id: &str) {
    EXPORTED.get_or_init(|| {
        // SAFETY: This function is called exactly once during worker startup
        // (guarded by `OnceLock`) before any concurrent async tasks are spawned
        // by the Tokio runtime. No data races with other threads are possible.
        unsafe {
            std::env::set_var("CODETETHER_SERVER", server);
            std::env::set_var("CODETETHER_WORKER_ID", worker_id);
            if let Some(token) = token {
                std::env::set_var("CODETETHER_TOKEN", token);
            }
        }
    });
}
