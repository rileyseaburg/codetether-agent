//! Decode only the indexed message and tool ranges requested by a reader.
use crate::provider::Message;
use crate::session::{Session, pages::PageKind};
use anyhow::Result;
use rusqlite::Connection;
use std::ops::Range;
pub(super) fn fill(
    session: &mut Session,
    db: &Connection,
    messages: Range<usize>,
    tools: Range<usize>,
) -> Result<()> {
    for body in super::read::read_window(db, &session.id, 0, messages.start, messages.end)? {
        let (message, page): (Message, PageKind) = serde_json::from_str(&body)?;
        session.messages.push(message);
        session.pages.push(page);
    }
    for body in super::read::read_window(db, &session.id, 1, tools.start, tools.end)? {
        session.tool_uses.push(serde_json::from_str(&body)?);
    }
    Ok(())
}
