//! Restore a consistent indexed window and its persistence checkpoint.
use crate::{Checkpoint, Restore, TailLoad};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::path::Path;

/// Restore an indexed session window with its original durable identity.
/// # Arguments
/// `path` selects the locator, `window` limits each record stream, and
/// `prepare` performs the application's legacy import before indexed reading.
/// # Returns
/// The restored session, excluded record count and locator byte length.
/// # Errors
/// Propagates recovery, import, SQLite, decoding, identity and file errors.
/// The entire indexed read occurs within one SQLite snapshot.
/// # Panics
/// Checkpoint poisoning or a native restore callback may panic.
/// # Examples
/// ```rust,no_run
/// # fn example<S: codetether_session_store::Restore>(path: &std::path::Path) -> anyhow::Result<()>
/// # where S::Tool: serde::de::DeserializeOwned {
/// let loaded = codetether_session_store::load_sync::<S>(path, 20, |_| Ok(()))?;
/// # Ok(()) }
/// ```
pub fn load_sync<S: Restore>(
    path: &Path,
    window: usize,
    prepare: impl FnOnce(&Path) -> Result<()>,
) -> Result<TailLoad<S>>
where
    S::Tool: DeserializeOwned,
{
    super::recover_locator::run(path)?;
    prepare(path)?;
    let id = crate::marker::id(path)?;
    let db = crate::connection::open(path)?;
    let tx = db.unchecked_transaction()?;
    let (revision, header, messages, tools) = crate::read::header(&tx, &id)?;
    let start = messages.saturating_sub(window);
    let tool_start = tools.saturating_sub(window);
    let mut session = S::decode_header(&header)?;
    anyhow::ensure!(session.id() == id, "session header identity mismatch");
    super::hydrate::fill(&mut session, &tx, start..messages, tool_start..tools)?;
    *session.state().0.lock().unwrap() = Checkpoint {
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
        constraints: crate::constraints::read(&tx, session.id(), start)?,
        pin_sources: Vec::new(),
    };
    super::finish_restore::finish(&mut session, &tx, start, messages)?;
    Ok(TailLoad {
        session,
        dropped: start + tool_start,
        file_bytes: std::fs::metadata(path)?.len(),
    })
}
