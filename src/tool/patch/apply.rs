//! File-level patch application.

#[path = "apply_commit.rs"]
mod commit;
#[path = "apply_prepare.rs"]
mod prepare;

use super::types::PatchHunk;
use anyhow::Result;
use std::path::Path;

pub(super) use prepare::Prepared;

/// Result of attempting to apply parsed hunks.
pub(super) struct ApplyOutcome {
    pub messages: Vec<String>,
    pub files_written: Vec<String>,
}

pub(super) fn prepare(root: &Path, hunks: &[PatchHunk]) -> Result<Prepared> {
    prepare::run(root, hunks)
}

pub(super) fn commit(prepared: Prepared, dry_run: bool) -> Result<ApplyOutcome> {
    commit::run(prepared, dry_run)
}
