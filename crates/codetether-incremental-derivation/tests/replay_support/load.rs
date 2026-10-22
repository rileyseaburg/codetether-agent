//! Read a consistent native transcript and existing summaries without migrations.
use super::Session;
use anyhow::{Context, Result};
use codetether_context_policy::{
    pages::PageKind,
    summary_index::{SummaryIndex, SummaryRange},
};
use codetether_provider_core::Message;
use codetether_session_store::{read, summaries};
use rusqlite::{Connection, OpenFlags, params};
use std::path::Path;

pub(crate) fn load(path: &Path) -> Result<(Session, String, usize)> {
    let id = path
        .file_stem()
        .and_then(|name| name.to_str())
        .context("invalid locator name")?;
    let db = Connection::open_with_flags(
        path.parent()
            .context("locator has no parent")?
            .join("sessions.sqlite3"),
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;
    let tx = db.unchecked_transaction()?;
    let (_, header, count, _) = read::header(&tx, id)?;
    let header: serde_json::Value = serde_json::from_str(&header)?;
    anyhow::ensure!(
        header["id"].as_str() == Some(id),
        "locator/header identity mismatch"
    );
    let model = header["metadata"]["model"].as_str().unwrap_or("glm-5.3");
    let model = model
        .split_once('/')
        .map_or(model, |(_, model)| model)
        .to_owned();
    let start = count.saturating_sub(1_000);
    let mut messages = Vec::new();
    let mut pages = Vec::new();
    for body in read::read_window(&tx, id, 0, start, count)? {
        let (message, page): (Message, PageKind) = serde_json::from_str(&body)?;
        messages.push(message);
        pages.push(page);
    }
    let summary_index = summaries::read(&tx, id, start, count)?;
    let mut background = SummaryIndex::new();
    let mut query = tx.prepare("SELECT seq,end_seq,body FROM projections WHERE session_id=?1 AND name='rlm' AND seq>=?2 AND end_seq<=?3 ORDER BY seq DESC LIMIT 128")?;
    let rows = query.query_map(params![id, start, count], |row| {
        Ok((
            row.get::<_, usize>(0)?,
            row.get::<_, usize>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    for row in rows {
        let (from, to, body) = row?;
        let body: serde_json::Value = serde_json::from_str(&body)?;
        if let Some(range) = SummaryRange::new(from - start, to - start) {
            background.insert(range, serde_json::from_value(body["node"].clone())?);
        }
    }
    Ok((
        Session {
            offset: start,
            messages,
            pages,
            summary_index,
            background,
        },
        model,
        count,
    ))
}
