//! Filesystem location of the intro ledger.

use std::path::PathBuf;

/// Where the ledger lives: `<data_dir>/a2a/introduced.json`.
pub(crate) fn ledger_path() -> Option<PathBuf> {
    crate::config::Config::data_dir().map(|d| d.join("a2a").join("introduced.json"))
}
