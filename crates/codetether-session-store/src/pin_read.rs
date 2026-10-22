//! Read sparse canonical sources under one consistent SQLite snapshot.
use crate::{Checkpoint, pin_edits::PinSource};
use anyhow::{Result, ensure};
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
pub(super) fn read(checkpoint: &Checkpoint, indices: &[usize]) -> Result<Vec<PinSource>> {
    let db = crate::connection::open(&checkpoint.path)?;
    let tx = db.unchecked_transaction()?;
    let (revision, _, count, _) = crate::read::header(&tx, &checkpoint.id)?;
    ensure!(
        revision == checkpoint.revision,
        "SESSION_REVISION_CONFLICT: reload before editing historical pins"
    );
    indices
        .iter()
        .map(|&index| {
            ensure!(
                index < count,
                "pin source index {index} is outside canonical history"
            );
            let rows = crate::read::read_window(&tx, &checkpoint.id, 0, index, index + 1)?;
            ensure!(rows.len() == 1, "canonical pin source {index} is missing");
            let (message, page): (Message, PageKind) = serde_json::from_str(&rows[0])?;
            Ok(PinSource {
                index,
                message,
                original: page,
                page,
            })
        })
        .collect()
}
