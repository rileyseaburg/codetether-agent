//! Decode native consumer snapshots inside the shared database transaction.
use super::cursor::Delivery;
use anyhow::Result;
use std::path::Path;
mod hydrate;
pub(super) fn next(path: &Path, id: &str, name: &str, limit: usize) -> Result<Option<Delivery>> {
    codetether_session_store::cursor_read::next(
        path,
        id,
        name,
        limit,
        super::migration::ensure,
        hydrate::session,
    )
}
