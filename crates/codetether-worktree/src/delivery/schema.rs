//! Versioned delivery-ledger schema.

use anyhow::{Result, ensure};
use rusqlite::Connection;

const VERSION: i64 = 1;

const SQL: &str = "
CREATE TABLE IF NOT EXISTS deliveries (
 id INTEGER PRIMARY KEY,
 created_at TEXT NOT NULL,
 target TEXT NOT NULL,
 branch TEXT NOT NULL,
 head TEXT NOT NULL,
 base TEXT NOT NULL,
 status TEXT NOT NULL,
 files TEXT NOT NULL,
 detail TEXT NOT NULL,
 patch BLOB NOT NULL);
CREATE INDEX IF NOT EXISTS delivery_target ON deliveries(target, id DESC);";

pub(super) fn upgrade(db: &Connection) -> Result<()> {
    let version: i64 = db.pragma_query_value(None, "user_version", |row| row.get(0))?;
    ensure!(
        version <= VERSION,
        "delivery ledger requires a newer runtime (schema {version})"
    );
    if version < VERSION {
        let tx = db.unchecked_transaction()?;
        tx.execute_batch(SQL)?;
        tx.pragma_update(None, "user_version", VERSION)?;
        tx.commit()?;
    }
    Ok(())
}
