//! Preserve host PATH order while admitting only mounted toolchain/system roots.

use std::ffi::OsStr;
use std::path::PathBuf;

pub(crate) fn path_entries(roots: &[PathBuf]) -> Vec<PathBuf> {
    path_entries_from(&std::env::var_os("PATH").unwrap_or_default(), roots)
}

pub(crate) fn path_entries_from(host: &OsStr, roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    for entry in std::env::split_paths(host) {
        let visible = super::SYSTEM.iter().any(|system| entry.starts_with(system))
            || roots.iter().any(|root| entry.starts_with(root));
        if visible && !entries.contains(&entry) {
            entries.push(entry);
        }
    }
    entries
}
