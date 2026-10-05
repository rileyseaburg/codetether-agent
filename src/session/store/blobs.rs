//! Immutable content-addressed payloads keep large tool outputs out of record indexes.
use anyhow::Result;
use rusqlite::{Connection, Transaction, params};
use sha2::{Digest, Sha256};
const PREFIX: &str = "@sha256:";
pub(super) fn put(tx: &Transaction<'_>, body: &str) -> Result<String> {
    if body.len() < 64 * 1024 {
        return Ok(body.to_owned());
    }
    let hash = hex::encode(Sha256::digest(body.as_bytes()));
    tx.execute(
        "INSERT OR IGNORE INTO blobs VALUES (?1,?2)",
        params![hash, body],
    )?;
    Ok(format!("{PREFIX}{hash}"))
}
pub(super) fn get(db: &Connection, reference: String) -> Result<String> {
    let Some(hash) = reference.strip_prefix(PREFIX) else {
        return Ok(reference);
    };
    let body: String = db.query_row("SELECT body FROM blobs WHERE hash=?1", [hash], |row| {
        row.get(0)
    })?;
    anyhow::ensure!(
        hex::encode(Sha256::digest(body.as_bytes())) == hash,
        "session blob integrity failure"
    );
    Ok(body)
}
