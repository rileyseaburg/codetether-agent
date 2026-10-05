//! SQLite setup on blocking workers, with short transactions.
use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;
pub(super) fn open(locator: &Path) -> Result<Connection> {
    let parent = locator.parent().context("session locator needs a parent")?;
    std::fs::create_dir_all(parent)?;
    let file = parent.join("sessions.sqlite3");
    let db = Connection::open(&file)?;
    db.busy_timeout(std::time::Duration::from_secs(5))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600))?;
    }
    db.pragma_update(None, "foreign_keys", "ON")?;
    db.pragma_update(None, "journal_mode", "WAL")?;
    db.pragma_update(None, "synchronous", "FULL")?;
    let version: i64 = db.pragma_query_value(None, "user_version", |row| row.get(0))?;
    anyhow::ensure!(version <= 2, "session database requires a newer runtime");
    if version < 2 {
        let tx = db.unchecked_transaction()?;
        super::upgrade::run(&tx, version)?;
        tx.pragma_update(None, "user_version", 2)?;
        tx.commit()?;
    }
    Ok(db)
}
