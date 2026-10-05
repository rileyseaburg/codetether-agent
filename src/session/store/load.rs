//! Indexed session resume; retaining a smaller window never changes identity.
use super::state::Checkpoint;
use crate::session::{Session, tail_load::TailLoad};
use anyhow::{Context, Result};
use std::path::Path;
pub(crate) async fn load(path: &Path, window: usize) -> Result<TailLoad> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || load_sync(&path, window))
        .await
        .context("session reader panicked")?
}
pub(super) fn load_sync(path: &Path, window: usize) -> Result<TailLoad> {
    super::recover_locator::run(path)?;
    super::migration::ensure(path)?;
    let id = super::marker::id(path)?;
    let db = super::connection::open(path)?;
    let tx = db.unchecked_transaction()?;
    let (revision, header, messages, tools) = super::read::header(&tx, &id)?;
    let start = messages.saturating_sub(window);
    let tool_start = tools.saturating_sub(window);
    let mut session: Session = super::header::decode(&header)?;
    anyhow::ensure!(session.id == id, "session header identity mismatch");
    super::hydrate::fill(&mut session, &tx, start..messages, tool_start..tools)?;
    *session.storage.0.lock().unwrap() = Checkpoint {
        id,
        path: path.into(),
        revision,
        read_only: false,
        message_start: start,
        tool_start,
        header,
        message_end: messages,
        tool_end: tools,
        versions: [0; 3],
        constraints: super::constraints::read(&tx, &session.id, start)?,
    };
    super::save::clean(&session);
    session.summary_index = super::summaries::read(&tx, &session.id, start, messages)?;
    session.normalize_sidecars();
    Ok(TailLoad {
        session,
        dropped: start + tool_start,
        file_bytes: std::fs::metadata(path)?.len(),
    })
}
