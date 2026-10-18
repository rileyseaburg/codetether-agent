//! One-time background repair for legacy sessions without recall sidecars.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

/// Schedule one bounded legacy backfill per workspace key when explicitly enabled.
/// # Arguments
/// Receives a workspace path and shared native worker host.
/// # Returns
/// Returns immediately; disabled or already scheduled workspaces do no work.
/// # Examples
/// ```no_run
/// # fn example(host: std::sync::Arc<impl codetether_recall_worker::Host>) {
/// codetether_recall_worker::schedule_backfill(std::path::Path::new("."), host);
/// # }
/// ```
pub fn schedule<H: crate::Host>(workspace: &Path, host: std::sync::Arc<H>) {
    if !super::backfill_policy::enabled() {
        return;
    }
    let workspace = codetether_recall_store::paths::canonical(workspace);
    let key = codetether_recall_store::paths::catalog(&workspace, host.as_ref())
        .unwrap_or_else(|_| workspace.clone());
    let start = scheduled()
        .lock()
        .map(|mut paths| paths.insert(key))
        .unwrap_or(false);
    if start {
        tokio::spawn(async move { super::backfill_worker::run(workspace, host.as_ref()).await });
    }
}

fn scheduled() -> &'static Mutex<HashSet<PathBuf>> {
    static SCHEDULED: OnceLock<Mutex<HashSet<PathBuf>>> = OnceLock::new();
    SCHEDULED.get_or_init(|| Mutex::new(HashSet::new()))
}
