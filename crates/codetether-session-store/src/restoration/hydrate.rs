//! Decode only the indexed message and tool ranges requested by a reader.
use crate::Restore;
use anyhow::Result;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
use rusqlite::Connection;
use serde::de::DeserializeOwned;
use std::ops::Range;
pub(crate) fn fill<S: Restore>(
    session: &mut S,
    db: &Connection,
    messages: Range<usize>,
    tools: Range<usize>,
) -> Result<()>
where
    S::Tool: DeserializeOwned,
{
    for body in crate::read::read_window(db, session.id(), 0, messages.start, messages.end)? {
        let (message, page): (Message, PageKind) = serde_json::from_str(&body)?;
        session.messages_mut().push(message);
        session.pages_mut().push(page);
    }
    for body in crate::read::read_window(db, session.id(), 1, tools.start, tools.end)? {
        session.tools_mut().push(serde_json::from_str(&body)?);
    }
    Ok(())
}
