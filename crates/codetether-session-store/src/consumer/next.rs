//! Bind protected import and snapshot hydration to a consumer transaction.
use crate::{Delivery, legacy::Legacy};
use std::path::Path;
/// Decode the next consumer range from an explicit session location.
/// # Arguments
/// `path` and `id` identify the session; `name` selects the cursor and `limit`
/// bounds its batch. `S` supplies the legacy decoder and snapshot fields.
/// # Returns
/// A read-only snapshot with an acknowledgement ticket, or `None` at the end.
/// # Errors
/// Returns import, database or decoding errors; rejects a zero batch limit.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_session_store::legacy::Legacy>(path: &std::path::Path) -> anyhow::Result<()> {
/// let _delivery = codetether_session_store::consumer::next::<S>(path, "session", "recall", 8)?;
/// # Ok(()) }
/// ```
pub fn next<S: Legacy>(
    path: &Path,
    id: &str,
    name: &str,
    limit: usize,
) -> anyhow::Result<Option<Delivery<S>>> {
    crate::cursor_read::next(
        path,
        id,
        name,
        limit,
        crate::legacy::ensure::<S>,
        crate::restore_consumer::<S>,
    )
}
