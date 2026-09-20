//! Process-wide reviewer runtime installed once at TUI startup.
//!
//! The approval event handler has no handle to the loaded configuration or
//! provider registry, so `run/config.rs` installs both here (mirroring
//! `tui::ui::trust_status`). Reads are cheap clones of `Arc`s.

use std::sync::{Arc, LazyLock, Mutex};

use crate::config::ReviewConfig;
use crate::provider::ProviderRegistry;

#[derive(Clone)]
pub struct ReviewRuntime {
    pub config: ReviewConfig,
    pub registry: Arc<ProviderRegistry>,
}

static RUNTIME: LazyLock<Mutex<Option<ReviewRuntime>>> = LazyLock::new(|| Mutex::new(None));

/// Install the reviewer settings and provider registry for this process.
pub fn install(config: ReviewConfig, registry: Arc<ProviderRegistry>) {
    *RUNTIME.lock().expect("review runtime lock") = Some(ReviewRuntime { config, registry });
}

/// The installed runtime, only when reviewing is enabled.
pub fn enabled() -> Option<ReviewRuntime> {
    RUNTIME
        .lock()
        .expect("review runtime lock")
        .clone()
        .filter(|runtime| runtime.config.enabled())
}

/// Forget the installed runtime (tests).
pub fn reset() {
    *RUNTIME.lock().expect("review runtime lock") = None;
}
