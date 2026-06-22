//! Path ↔ `file://` URI conversion helpers for the LSP client.
//!
//! Kept in a dedicated module so the conversion logic has a single home and
//! `client.rs` stays focused on the JSON-RPC client itself.

use std::path::{Path, PathBuf};

/// Converts a filesystem path to a `file://` URI (canonicalized when possible).
pub fn path_to_uri(path: &Path) -> String {
    let absolute = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    format!("file://{}", absolute.display())
}

/// Converts a `file://` URI back into a filesystem path.
pub fn uri_to_path(uri: &str) -> PathBuf {
    PathBuf::from(uri.strip_prefix("file://").unwrap_or(uri))
}
