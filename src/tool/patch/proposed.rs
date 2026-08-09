//! In-memory proposed file contents for pre-approval analysis.

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::{file_io, group, hunk_apply, parser, path_guard};

pub(crate) fn contents(root: &Path, patch: &str) -> Result<Vec<(PathBuf, String)>> {
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
