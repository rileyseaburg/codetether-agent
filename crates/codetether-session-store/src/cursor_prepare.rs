//! Create missing cursor rows before entering a read-only snapshot.
//! Existing rows are found by a reader, so routine polling never queues behind
//! the cross-process writer lock.
use anyhow::Result;
use rusqlite::{OptionalExtension, params};
use std::path::Path;

pub(crate) fn exists(path: &Path, id: &str, name: &str) -> Result<bool> {
    let db = crate::connection::open(path)?;
    let found = db
        .query_row(
            "SELECT 1 FROM consumers WHERE session_id=?1 AND name=?2",
            params![id, name],
            |_| Ok(()),
        )
        .optional()?;
    Ok(found.is_some())
}

pub(crate) fn ensure(path: &Path, id: &str, name: &str) -> Result<()> {
    if exists(path, id, name)? {
        return Ok(());
    }
    let id = id.to_owned();
    let name = name.to_owned();
    crate::coordinator::write(path, move |db| {
        db.execute(
            "INSERT OR IGNORE INTO consumers VALUES (?1,?2,0,0)",
            params![id, name],
        )?;
        Ok(())
    })
}
