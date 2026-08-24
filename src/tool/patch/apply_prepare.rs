//! Read-only validation and in-memory application of every patch hunk.

use super::super::{file_io, group, hunk_apply, path_guard, types::PatchHunk};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub(crate) struct Prepared {
    pub(super) files: Vec<PreparedFile>,
    pub(super) messages: Vec<String>,
}

pub(super) struct PreparedFile {
    pub(super) name: String,
    pub(super) path: PathBuf,
    pub(super) content: String,
}

pub(super) fn run(root: &Path, hunks: &[PatchHunk]) -> Result<Prepared> {
    let mut files = Vec::new();
    let mut messages = Vec::new();
    for (name, file_hunks) in group::by_file(hunks) {
        let path = path_guard::resolve(root, &name)?;
        let mut content = file_io::read_existing(&path, &name)?;
        for hunk in file_hunks {
            content = hunk_apply::apply(&content, hunk)
                .with_context(|| format!("failed to apply hunk to {name}"))?;
            messages.push(format!(
                "✓ Applied hunk to {} at line {}",
                name, hunk.start_line
            ));
        }
        files.push(PreparedFile {
            name,
            path,
            content,
        });
    }
    Ok(Prepared { files, messages })
}
