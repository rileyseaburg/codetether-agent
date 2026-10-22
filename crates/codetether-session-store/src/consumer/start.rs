//! Initialize a new derived-view cursor near the current working tail.
use anyhow::Result;
use rusqlite::params;
use std::path::Path;
/// Initialize a missing consumer at an aligned tail boundary.
/// Existing consumers are never advanced or rewound by this operation.
/// Callers must import legacy storage before invoking this synchronous helper.
/// # Arguments
/// `path` and `id` identify storage; `name` identifies the derived view.
/// `tail` is the minimum recent message count to replay, and `alignment`
/// rounds the starting position down to a complete processing block.
/// # Returns
/// Unit after an atomic insert-if-absent; all source records remain untouched.
/// # Errors
/// Rejects zero or out-of-range sizes and propagates SQLite errors.
/// # Examples
/// ```no_run
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// codetether_session_store::consumer::start_at_tail(path, "session", "view:v2", 128, 16)?;
/// # Ok(()) }
/// ```
pub fn start_at_tail(
    path: &Path,
    id: &str,
    name: &str,
    tail: usize,
    alignment: usize,
) -> Result<()> {
    anyhow::ensure!(
        tail > 0 && alignment > 0,
        "cursor tail and alignment must be nonzero"
    );
    let tail = i64::try_from(tail)?;
    let alignment = i64::try_from(alignment)?;
    let db = crate::connection::open(path)?;
    db.execute(
        "INSERT OR IGNORE INTO consumers(session_id,name,seq,revision)
         SELECT id,?2,(MAX(message_count-?3,0)/?4)*?4,0 FROM sessions WHERE id=?1",
        params![id, name, tail, alignment],
    )?;
    Ok(())
}
