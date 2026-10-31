//! Create missing cursor rows before entering a read-only snapshot.
use anyhow::Result;
use std::path::Path;

pub(crate) fn ensure(path: &Path, id: &str, name: &str) -> Result<()> {
    let id = id.to_owned();
    let name = name.to_owned();
    crate::coordinator::write(path, move |db| {
        db.execute(
            "INSERT OR IGNORE INTO consumers VALUES (?1,?2,0,0)",
            rusqlite::params![id, name],
        )?;
        Ok(())
    })
}
