//! Disposable test inputs inside the workspace, not system temporary roots.

// Each integration-test binary uses a different subset of these constructors.
#![allow(dead_code)]

use std::io;
use std::path::PathBuf;

fn root() -> io::Result<PathBuf> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("artifacts")
        .join("test-fixtures");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

/// Create a unique workspace-local directory for tool-boundary tests.
pub fn tempdir() -> io::Result<tempfile::TempDir> {
    tempfile::Builder::new().prefix("tool-").tempdir_in(root()?)
}

/// Create a unique workspace-local file for edit matcher tests.
pub fn file() -> io::Result<tempfile::NamedTempFile> {
    tempfile::Builder::new()
        .prefix("matcher-")
        .tempfile_in(root()?)
}
