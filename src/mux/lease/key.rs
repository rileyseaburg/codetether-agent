//! Hash identity and hierarchy comparison for leased paths.

use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(super) struct LeaseKey {
    pub workspace: PathBuf,
    pub path: PathBuf,
}

pub(super) fn overlaps(left: &Path, right: &Path) -> bool {
    left.as_os_str().is_empty()
        || right.as_os_str().is_empty()
        || left.starts_with(right)
        || right.starts_with(left)
}
