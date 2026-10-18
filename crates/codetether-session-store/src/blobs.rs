//! Immutable content-addressed payloads keep large tool outputs out of record indexes.
use anyhow::Result;
use rusqlite::{Connection, Transaction, params};
use sha2::{Digest, Sha256};
const PREFIX: &str = "@sha256:";
/// Store a large body by content hash, retaining small bodies inline.
/// # Arguments
/// `tx` is the active write transaction; `body` is the serialized payload.
/// # Returns
/// An inline body or a content-addressed reference.
/// # Errors
/// SQLite insertion fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::blobs::put;
/// # fn example(tx: &rusqlite::Transaction<'_>) -> anyhow::Result<()> {
/// let result = put(tx, "payload")?;
/// # Ok(()) }
/// ```
pub fn put(tx: &Transaction<'_>, body: &str) -> Result<String> {
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
/// Resolve an inline body or verify and load a content-addressed payload.
/// # Arguments
/// `db` is the open database; `reference` is a stored record value.
/// # Returns
/// The original payload with large-blob integrity verified.
/// # Errors
/// SQLite lookup fails or the content hash does not match.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::blobs::get;
/// # fn example(db: &rusqlite::Connection) -> anyhow::Result<()> {
/// let result = get(db, "inline".into())?;
/// # Ok(()) }
/// ```
pub fn get(db: &Connection, reference: String) -> Result<String> {
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
