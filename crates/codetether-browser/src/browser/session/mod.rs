//! Browser session facade.
//!
//! This module exposes the stable session type while delegating execution to
//! the native TetherScript backend when that feature is enabled.

#[cfg(feature = "tetherscript")]
mod native;
mod runtime;
mod state;

/// Cloneable browser session handle.
pub use state::BrowserSession;

/// Return the detected external browser executable, if one is required.
///
/// Native TetherScript browserctl does not depend on Chromium or another
/// external browser process, so this currently returns `None`.
pub fn detect_browser() -> Option<std::path::PathBuf> {
    None
}
