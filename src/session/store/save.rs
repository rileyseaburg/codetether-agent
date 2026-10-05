//! Compatibility save boundary; only dirty records cross to the writer thread.
pub(super) use super::checkpoint::clean;
use super::state::Checkpoint;
use crate::session::Session;
use anyhow::{Context, Result};
use std::path::Path;
pub(crate) async fn save(session: &Session, path: &Path) -> Result<bool> {
    super::id::validate(&session.id)?;
    let mut checkpoint = session.storage.0.lock().unwrap().clone();
    if checkpoint.id != session.id || checkpoint.path != path {
        checkpoint = Checkpoint {
            id: session.id.clone(),
            path: path.into(),
            ..Default::default()
        };
    }
    let batch = super::batch::prepare(session, &checkpoint)?;
    anyhow::ensure!(
        !checkpoint.read_only,
        "cannot save a historical consumer range as a session"
    );
    if batch.unchanged(&checkpoint) {
        return Ok(false);
    }
    let owned_path = path.to_path_buf();
    let header = batch.header.clone();
    let revision = tokio::task::spawn_blocking(move || {
        let ends = (batch.messages.end, batch.tools.end);
        super::marker::check(&owned_path, &batch.id)?;
        let revision = super::transaction::commit(&owned_path, &batch)?;
        super::marker::create(&owned_path, &batch.id)?;
        anyhow::Ok((revision, ends))
    })
    .await
    .context("session writer panicked")??;
    checkpoint.revision = revision.0;
    checkpoint.message_end = revision.1.0;
    checkpoint.tool_end = revision.1.1;
    checkpoint.header = header;
    *session.storage.0.lock().unwrap() = checkpoint;
    clean(session);
    Ok(true)
}
