//! File-level patch application.

use super::{file_io, group, hunk_apply, path_guard, types::PatchHunk};
use anyhow::Result;
use std::path::Path;

/// Result of attempting to apply parsed hunks.
pub(super) struct ApplyOutcome {
    pub messages: Vec<String>,
    pub files_written: Vec<String>,
}

/// Apply hunks to files, or simulate the same work when `dry_run` is true.
pub(super) fn run(root: &Path, hunks: &[PatchHunk], dry_run: bool) -> Result<ApplyOutcome> {
    let mut outcome = ApplyOutcome {
        messages: Vec::new(),
        files_written: Vec::new(),
    };
    for (file, file_hunks) in group::by_file(hunks) {
        let path = path_guard::resolve(root, &file)?;
        let mut content = file_io::read_existing(&path, &file)?;
        for hunk in file_hunks {
            match hunk_apply::apply(&content, hunk) {
                Ok(new_content) => {
                    content = new_content;
                    outcome.messages.push(format!(
                        "✓ Applied hunk to {} at line {}",
                        file, hunk.start_line
                    ));
                }
                Err(error) => {
                    outcome
                        .messages
                        .push(format!("✗ Failed to apply hunk to {}: {}", file, error));
                }
            }
        }
        if !dry_run {
            file_io::write_updated(&path, &content)?;
            outcome.files_written.push(file);
        }
    }
    Ok(outcome)
}
