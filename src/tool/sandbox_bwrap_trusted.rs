//! Resolve Bubblewrap only from administrator-controlled system locations.

use std::path::{Path, PathBuf};

const CANDIDATES: &[&str] = &["/usr/bin/bwrap", "/bin/bwrap", "/usr/local/bin/bwrap"];

pub(super) fn find() -> Option<PathBuf> {
    CANDIDATES.iter().find_map(|candidate| trusted(Path::new(candidate)))
}

#[cfg(unix)]
fn trusted(path: &Path) -> Option<PathBuf> {
    use std::os::unix::fs::MetadataExt;
    let resolved = path.canonicalize().ok()?;
    let metadata = resolved.metadata().ok()?;
    (metadata.is_file() && metadata.uid() == 0 && metadata.mode() & 0o022 == 0)
        .then_some(resolved)
}

#[cfg(not(unix))]
fn trusted(_path: &Path) -> Option<PathBuf> {
    None
}

#[cfg(test)]
#[path = "sandbox_bwrap_trusted_tests.rs"]
mod tests;