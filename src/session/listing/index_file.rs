//! Append-only session listing index (`<sessions_dir>/index.jsonl`).
//!
//! One [`SessionSummary`] per line. Duplicate lines per id are tolerated;
//! the last line for a given id wins when read.
//!
//! Layout:
//! - [`path`] — on-disk path of `index.jsonl`.
//! - [`append_async`] — best-effort append used by `Session::save`.
//! - [`read_sync`] lives in [`super::index_file_io`].

use std::path::PathBuf;

use anyhow::Result;
use tokio::fs;

use super::index_file_io::append_sync;
use super::summary::SessionSummary;

pub(super) const INDEX_FILENAME: &str = "index.jsonl";

/// Resolve the on-disk path of the listing index.
pub(super) fn path() -> Result<PathBuf> {
    Ok(super::directory::sessions_dir()?.join(INDEX_FILENAME))
}

/// Append a single summary line to `index.jsonl`. O_APPEND, blocking pool,
/// best-effort. A failure here is logged at debug, never propagated.
/// `pub(crate)` so [`crate::session::save_index`] can call it from
/// `Session::save`.
pub(crate) async fn append_async(summary: &SessionSummary) {
    let Ok(p) = path() else { return };
    if let Some(parent) = p.parent() {
        let _ = fs::create_dir_all(parent).await;
    }
    let s = summary.clone();
    let _ = tokio::task::spawn_blocking(move || append_sync(&p, &s)).await;
}
