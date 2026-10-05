//! Consumer snapshots are range-bound and cannot accidentally overwrite history.
use super::super::cursor::Ticket;
use crate::session::Session;
use anyhow::Result;
use rusqlite::Transaction;
pub(super) fn session(
    tx: &Transaction<'_>,
    ticket: &Ticket,
    header: String,
    count: usize,
    revision: i64,
) -> Result<Session> {
    let mut session = super::super::header::decode(&header)?;
    for body in super::super::read::read_window(tx, &ticket.id, 0, ticket.from, ticket.to)? {
        let (message, page) = serde_json::from_str(&body)?;
        session.messages.push(message);
        session.pages.push(page);
    }
    {
        let mut state = session.storage.0.lock().unwrap();
        state.read_only = true;
        state.id = ticket.id.clone();
        state.path = ticket.path.clone();
        state.revision = revision;
        state.message_start = ticket.from;
        state.message_end = count;
        state.header = header;
    }
    super::super::save::clean(&session);
    Ok(session)
}
