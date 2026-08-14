//! Writable and protected path selection for macOS Seatbelt profiles.

use super::super::SandboxPolicy;
use super::base::PROTECTED;
use super::roots;
use std::path::{Path, PathBuf};

/// Absolute roots the confined process may write beneath.
///
/// Policy roots come first, then the implicit roots (sandbox `HOME`, the temp
/// directory, and the working directory). Every root is emitted both as given
/// and symlink-resolved so SBPL `subpath` rules match at enforcement time.
pub(super) fn writable_roots(
    policy: &SandboxPolicy,
    work_dir: &Path,
    temp_dir: &Path,
) -> Vec<String> {
    let candidates = policy
        .allowed_paths
        .iter()
        .cloned()
        .chain(roots::implicit(work_dir, temp_dir));
    let mut out: Vec<String> = candidates
        .filter(|path| path.is_absolute())
        .flat_map(|path| roots::resolved(&path))
        .map(display)
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Paths denied write access even though they sit inside a writable root.
pub(super) fn protected_paths(policy: &SandboxPolicy) -> Vec<String> {
    policy
        .allowed_paths
        .iter()
        .filter(|path| path.is_absolute())
        .flat_map(|root| PROTECTED.iter().map(move |name| root.join(name)))
        .flat_map(|path| roots::resolved(&path))
        .map(display)
        .collect()
}

fn display(path: PathBuf) -> String {
    path.display().to_string()
}
