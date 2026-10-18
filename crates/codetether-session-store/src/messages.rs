//! Indexed message windows with protected legacy import and typed decoding.
use crate::legacy::Legacy;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;

/// Read a bounded range of messages from a durable session.
/// # Arguments
/// `id` identifies the session; `start` is the inclusive absolute offset and
/// `limit` caps the number of decoded records. `S` supplies legacy import decoding.
/// # Returns
/// Chronologically ordered messages from the requested range.
/// # Errors
/// Returns path resolution, legacy import, SQLite, decoding or task-join errors.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_store::legacy::Legacy>() -> anyhow::Result<()> {
/// let page = codetether_session_store::read_messages::<S>("session-id", 100, 20).await?;
/// assert!(page.len() <= 20);
/// # Ok(()) }
/// ```
pub async fn read_messages<S: Legacy>(
    id: &str,
    start: usize,
    limit: usize,
) -> anyhow::Result<Vec<Message>> {
    let path = crate::lifecycle::session_path(id)?;
    let id = id.to_owned();
    tokio::task::spawn_blocking(move || {
        crate::legacy::ensure::<S>(&path)?;
        let db = crate::connection::open(&path)?;
        let rows = crate::read::read_window(&db, &id, 0, start, start.saturating_add(limit))?;
        rows.into_iter()
            .map(|body| {
                let (message, _): (Message, PageKind) = serde_json::from_str(&body)?;
                anyhow::Ok(message)
            })
            .collect()
    })
    .await?
}
