//! In-memory proposed file contents for pre-approval analysis.
//!
//! [contents] applies parsed hunks in memory and returns target paths and text.
//! It uses the same path guard and hunk application as patch execution.

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::{file_io, group, hunk_apply, parser, path_guard};

/// Reconstruct the result of a unified diff without writing files.
///
/// # Arguments
/// * `root` - Workspace directory for patch targets.
/// * `patch` - Unified diff text.
/// # Returns
/// Target paths and their proposed file contents.
/// # Errors
/// Returns an error for unsafe paths, failed reads or invalid hunks.
/// # Examples
/// ```rust
/// let files = codetether_patch::proposed::contents(std::path::Path::new("."), "").unwrap();
/// assert!(files.is_empty());
/// ```
pub fn contents(root: &Path, patch: &str) -> Result<Vec<(PathBuf, String)>> {
    let hunks = parser::parse_patch(patch);
    let mut files = Vec::new();
    for (file, file_hunks) in group::by_file(&hunks) {
        let path = path_guard::resolve(root, &file)?;
        let mut content = file_io::read_existing(&path, &file)?;
        for hunk in file_hunks {
            content = hunk_apply::apply(&content, hunk)?;
        }
        files.push((path, content));
    }
    Ok(files)
}

#[cfg(test)]
#[path = "proposed_tests.rs"]
mod tests;
