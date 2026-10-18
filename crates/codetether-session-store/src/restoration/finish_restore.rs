//! Complete native sidecars only after a consistent record window is restored.
use crate::Restore;
use anyhow::Result;
use rusqlite::Connection;
pub(crate) fn finish<S: Restore>(
    session: &mut S,
    db: &Connection,
    start: usize,
    end: usize,
) -> Result<()> {
    crate::clean(session);
    session.set_summary_index(crate::summaries::read(db, session.id(), start, end)?);
    session.normalize_sidecars();
    Ok(())
}
