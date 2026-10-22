//! Compatibility save boundary; only dirty records cross to the writer thread.
use super::state::Checkpoint;
use crate::Snapshot;
use crate::checkpoint::clean;
use anyhow::{Context, Result};
use std::path::Path;
/// Commit dirty session data and then acknowledge its persisted checkpoint.
/// # Arguments
/// `session` supplies native data and state; `path` selects the session locator.
/// # Returns
/// True after a write, or false when the prepared delta is unchanged.
/// # Errors
/// Returns identity, encoding, read-only, revision, database, locator or worker
/// errors. The checkpoint is not advanced before successful persistence.
/// # Panics
/// Panics if checkpoint synchronization is poisoned or no Tokio runtime exists.
/// # Examples
/// ```rust,no_run
/// # async fn example<S: codetether_session_store::Snapshot + Sync>(session: &S, path: &std::path::Path) -> anyhow::Result<()> {
/// let changed = codetether_session_store::save(session, path).await?;
/// # Ok(()) }
/// ```
pub async fn save<S: Snapshot + Sync>(session: &S, path: &Path) -> Result<bool> {
    super::id::validate(session.id())?;
    let mut checkpoint = session.state().0.lock().unwrap().clone();
    if checkpoint.id != session.id() || checkpoint.path != path {
        checkpoint = Checkpoint {
            id: session.id().to_owned(),
            path: path.into(),
            ..Default::default()
        };
    }
    let batch = crate::prepare(session, &checkpoint)?;
    let edits: Vec<_> = checkpoint
        .pin_sources
        .iter()
        .filter(|source| source.original != source.page)
        .cloned()
        .collect();
    anyhow::ensure!(
        !checkpoint.read_only,
        "cannot save a historical consumer range as a session"
    );
    if edits.is_empty() && batch.unchanged(&checkpoint) {
        return Ok(false);
    }
    let owned_path = path.to_path_buf();
    let header = batch.header.clone();
    let committed_edits = edits.clone();
    let revision = tokio::task::spawn_blocking(move || {
        let ends = (batch.messages.end, batch.tools.end);
        super::marker::check(&owned_path, &batch.id)?;
        let revision = super::transaction::commit_with_pins(&owned_path, &batch, &edits)?;
        super::marker::create(&owned_path, &batch.id)?;
        anyhow::Ok((revision, ends))
    })
    .await
    .context("session writer panicked")??;
    checkpoint.revision = revision.0;
    checkpoint.message_end = revision.1.0;
    checkpoint.tool_end = revision.1.1;
    checkpoint.header = header;
    for edit in committed_edits {
        checkpoint
            .constraints
            .retain(|source| source.source_turn != edit.index);
        if let Some(source) =
            crate::constraints::ConstraintRecord::from_message(edit.index, &edit.message, edit.page)
        {
            checkpoint.constraints.push(source);
        }
    }
    checkpoint
        .constraints
        .sort_by_key(|source| source.source_turn);
    checkpoint.pin_sources.clear();
    *session.state().0.lock().unwrap() = checkpoint;
    clean(session);
    Ok(true)
}
