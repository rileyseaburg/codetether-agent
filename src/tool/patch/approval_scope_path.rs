//! Canonical workspace paths used by patch approval resources.

use std::path::{Path, PathBuf};

pub(super) fn absolute(root: &Path) -> PathBuf {
    root.canonicalize().unwrap_or_else(|_| {
        root.is_absolute()
            .then(|| root.to_path_buf())
            .unwrap_or_else(|| current().join(root))
    })
}

pub(super) fn current() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
