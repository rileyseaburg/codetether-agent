//! Read canonical user evidence without loading or mutating a live session.
use anyhow::Result;
use codetether_provider_core::{ContentPart, Message};
use rusqlite::{Connection, OpenFlags, params};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Text and identity of a canonical user message.
/// # Examples
/// ```
/// let source = codetether_session_store::user_source::UserSource {
///     index: 4, text: "Inspect the CLI".into(), text_hash: "digest".into(),
/// };
/// assert_eq!(source.index, 4);
/// ```
pub struct UserSource {
    /// Absolute index in durable message history.
    pub index: usize,
    /// Text parts joined with newlines, without trimming or rewriting.
    pub text: String,
    /// Lowercase hexadecimal SHA-256 of `text`.
    pub text_hash: String,
}

/// Read the latest nonempty user message, excluding automatic goal reminders.
/// # Arguments
/// * `path` — Canonical session locator beside the indexed database.
/// * `id` — Session identity to search.
/// # Returns
/// The latest usable source among 64 user messages within the newest 1,000
/// record positions, or none. Older history is not scanned.
/// Missing databases are not created and legacy sessions are not migrated.
/// # Errors
/// Returns database, blob integrity, or message decoding errors.
/// # Examples
/// ```no_run
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let _source = codetether_session_store::user_source::read(path, "session-123")?;
/// # Ok(()) }
/// ```
pub fn read(path: &Path, id: &str) -> Result<Option<UserSource>> {
    let Some(parent) = path.parent() else {
        return Ok(None);
    };
    let file = parent.join("sessions.sqlite3");
    if !file.is_file() {
        return Ok(None);
    }
    let db = Connection::open_with_flags(file, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut query = db.prepare(
        "SELECT r.seq,r.body FROM records r
        LEFT JOIN blobs b ON b.hash=substr(r.body,9) AND substr(r.body,1,8)='@sha256:'
        WHERE r.session_id=?1 AND r.kind=0
        AND r.seq >= (SELECT COALESCE(MAX(seq),0) FROM records WHERE session_id=?1 AND kind=0)-?2+1
        AND json_extract(COALESCE(b.body,r.body),'$[0].role')='user'
        ORDER BY r.seq DESC LIMIT 64",
    )?;
    let rows = query.query_map(params![id, crate::window::WINDOW as i64], |row| {
        Ok((row.get::<_, usize>(0)?, row.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (index, body) = row?;
        let body = crate::blobs::get(&db, body)?;
        let (message, _): (Message, serde_json::Value) = serde_json::from_str(&body)?;
        let text = message
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        if text.trim().is_empty()
            || (text
                .trim()
                .starts_with("Continue working toward the active thread goal.")
                && text.contains("<objective>"))
        {
            continue;
        }
        let text_hash = hex::encode(Sha256::digest(text.as_bytes()));
        return Ok(Some(UserSource {
            index,
            text,
            text_hash,
        }));
    }
    Ok(None)
}
