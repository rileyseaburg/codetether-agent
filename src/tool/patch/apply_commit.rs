//! Commit of a completely preflighted patch plan.

use super::super::file_io;
use super::{ApplyOutcome, Prepared};
use anyhow::Result;

pub(super) fn run(prepared: Prepared, dry_run: bool) -> Result<ApplyOutcome> {
    let mut files_written = Vec::new();
    if !dry_run {
        for file in &prepared.files {
            file_io::write_updated(&file.path, &file.content)?;
            files_written.push(file.name.clone());
        }
    }
    Ok(ApplyOutcome {
        messages: prepared.messages,
        files_written,
    })
}
