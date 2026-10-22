//! Fold a consistent canonical transcript with bounded raw-message residency.
use crate::legacy::Legacy;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;

/// Fold every canonical message from one SQLite read transaction.
/// # Arguments
/// `path` is the session locator; `id` selects its durable identity;
/// `state` and `fold` supply the accumulator.
/// # Returns
/// Accumulated state and the absolute message count from the same snapshot.
/// Only one decoded raw message is retained at a time.
/// # Errors
/// Returns import, database, missing-record, decoding, callback or join errors.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_store::legacy::Legacy>(path: &std::path::Path) -> anyhow::Result<()> {
/// let (count, total) = codetether_session_store::fold_messages::<S, _, _>(
///     path, "session", 0usize, |count, _, _| { *count += 1; Ok(()) }).await?;
/// assert_eq!(count, total);
/// # Ok(()) }
/// ```
pub async fn fold_messages<S, T, F>(
    path: &std::path::Path,
    id: &str,
    mut state: T,
    mut fold: F,
) -> anyhow::Result<(T, usize)>
where
    S: Legacy,
    T: Send + 'static,
    F: FnMut(&mut T, usize, Message) -> anyhow::Result<()> + Send + 'static,
{
    let path = path.to_path_buf();
    let id = id.to_owned();
    tokio::task::spawn_blocking(move || {
        crate::legacy::ensure::<S>(&path)?;
        let mut db = crate::connection::open(&path)?;
        let tx = db.transaction()?;
        let (_, _, total, _) = crate::read::header(&tx, &id)?;
        let mut query =
            tx.prepare("SELECT seq,body FROM records WHERE session_id=?1 AND kind=0 ORDER BY seq")?;
        let mut rows = query.query([&id])?;
        let mut count = 0;
        while let Some(row) = rows.next()? {
            let index: usize = row.get(0)?;
            anyhow::ensure!(
                index == count,
                "canonical message gap: expected {count}, got {index}"
            );
            let body = crate::blobs::get(&tx, row.get(1)?)?;
            let (message, _): (Message, PageKind) = serde_json::from_str(&body)?;
            fold(&mut state, index, message)?;
            count += 1;
        }
        anyhow::ensure!(
            count == total,
            "canonical message count mismatch: expected {total}, got {count}"
        );
        Ok((state, total))
    })
    .await?
}
