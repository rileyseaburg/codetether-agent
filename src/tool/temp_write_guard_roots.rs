//! Hard-coded temporary-directory roots that agents may never write into.

use std::path::PathBuf;

/// Absolute temp roots baked into the binary.
///
/// This list is intentionally not configurable. Scratch files in a shared
/// temp directory escape the workspace, survive worktree cleanup, and hide
/// work from review, so writing there is treated as a hard error rather than
/// a policy decision.
pub(super) const DENIED: &[&str] = &[
    "/tmp",
    "/var/tmp",
    "/dev/shm",
    "/private/tmp",
    "/private/var/tmp",
    "/private/var/folders",
    "/var/folders",
    "/run/shm",
    "/usr/tmp",
];

/// Path segments that mark a Windows temp location.
pub(super) const DENIED_WINDOWS_SEGMENTS: &[&str] = &["\\appdata\\local\\temp", "\\windows\\temp"];

/// Every denied root, including temp locations discovered at runtime.
///
/// `TMPDIR` and [`std::env::temp_dir`] are consulted so a redirected temp
/// directory cannot be used to sidestep the hard-coded list.
pub(super) fn all() -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = DENIED.iter().map(PathBuf::from).collect();
    roots.push(std::env::temp_dir());
    roots.extend(env_roots());
    roots.sort();
    roots.dedup();
    roots
}

fn env_roots() -> Vec<PathBuf> {
    ["TMPDIR", "TMP", "TEMP"]
        .iter()
        .filter_map(std::env::var_os)
        .map(PathBuf::from)
        .collect()
}
