//! Application data-directory selection for cognition state.
use std::path::PathBuf;
// 100 MB

/// Get the persistence file path.
pub(super) fn state_path() -> PathBuf {
    let base = codetether_config::Config::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp/codetether-agent"));
    base.join("cognition").join("state.json")
}
