//! Remove stale Seatbelt profiles left behind by earlier sandboxed runs.

use std::path::Path;
use std::time::{Duration, SystemTime};

/// Profiles older than this are assumed to belong to finished processes.
const MAX_AGE: Duration = Duration::from_secs(3600);

/// Delete `codetether-seatbelt-*.sb` files in `dir` older than [`MAX_AGE`].
///
/// Staged profiles must outlive the plan that references them, so they cannot
/// be deleted on drop. Pruning on each staging call bounds the temp-dir
/// footprint without racing a live `sandbox-exec` invocation.
pub(super) fn stale(dir: &Path, now: SystemTime) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if is_profile(&path) && expired(&entry, now) {
            let _ = std::fs::remove_file(&path);
        }
    }
}

fn is_profile(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("codetether-seatbelt-") && name.ends_with(".sb"))
}

fn expired(entry: &std::fs::DirEntry, now: SystemTime) -> bool {
    entry
        .metadata()
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|modified| now.duration_since(modified).ok())
        .is_some_and(|age| age > MAX_AGE)
}

#[cfg(test)]
#[path = "sandbox_seatbelt_prune_tests.rs"]
mod tests;