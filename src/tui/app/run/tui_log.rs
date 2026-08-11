//! Persistent TUI diagnostic log access.

use std::fs::{File, OpenOptions};
use std::path::Path;

/// Opens the workspace TUI log without discarding earlier diagnostics.
///
/// # Arguments
///
/// * `directory` — Existing data directory containing `tui.log`.
///
/// # Returns
///
/// An append-only file handle, or `None` when the log cannot be opened.
///
/// # Examples
///
/// ```text
/// previous launch diagnostics
/// current launch diagnostics
/// ```
pub fn open(directory: &Path) -> Option<File> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(directory.join("tui.log"))
        .ok()
}

#[cfg(test)]
#[path = "tui_log_tests.rs"]
mod tests;
