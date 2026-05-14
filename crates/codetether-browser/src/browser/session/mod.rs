#[cfg(feature = "tetherscript")]
mod native;
mod runtime;
mod state;

pub use state::BrowserSession;

pub fn detect_browser() -> Option<std::path::PathBuf> {
    None
}
