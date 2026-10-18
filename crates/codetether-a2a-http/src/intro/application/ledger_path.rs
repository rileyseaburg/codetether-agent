//! Filesystem location of the intro ledger.

use std::path::PathBuf;

/// Where the ledger lives: `<data_dir>/a2a/introduced.json`.
/// # Arguments
/// Reads the application's configured data directory.
/// # Returns
/// The ledger path, or `None` if no data directory is available.
/// # Examples
/// ```
/// let _path = codetether_a2a_http::intro::application::ledger::ledger_path();
/// ```
pub fn ledger_path() -> Option<PathBuf> {
    codetether_config::Config::data_dir().map(|d| d.join("a2a").join("introduced.json"))
}
