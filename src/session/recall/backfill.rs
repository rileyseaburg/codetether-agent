//! One-time background repair for legacy sessions without recall sidecars.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

pub(super) fn schedule(workspace: &Path) {
    let workspace = super::paths::canonical(workspace);
    let key = super::paths::catalog(&workspace).unwrap_or_else(|_| workspace.clone());
    let start = scheduled()
        .lock()
        .map(|mut paths| paths.insert(key))
        .unwrap_or(false);
    if start {
        tokio::spawn(super::backfill_worker::run(workspace));
    }
}

fn scheduled() -> &'static Mutex<HashSet<PathBuf>> {
    static SCHEDULED: OnceLock<Mutex<HashSet<PathBuf>>> = OnceLock::new();
    SCHEDULED.get_or_init(|| Mutex::new(HashSet::new()))
}
