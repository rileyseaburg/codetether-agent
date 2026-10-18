//! Consumer snapshots are range-bound and cannot accidentally overwrite history.
use crate::{Restore, Ticket};
use anyhow::Result;
use rusqlite::Transaction;
/// Decode a read-only consumer range using the caller's database snapshot.
/// # Arguments
/// `tx` supplies the snapshot; `ticket` identifies its range. `header`,
/// `count` and `revision` come from the same session row in that snapshot.
/// # Returns
/// A decoded session with read-only checkpoint state and clean loaded vectors.
/// # Errors
/// Returns header/record decoding and database read errors.
/// # Panics
/// Panics on checkpoint poisoning or native decoder panics.
/// # Examples
/// ```rust,no_run
/// # fn example<S: codetether_session_store::Restore>(tx: &rusqlite::Transaction<'_>, ticket: &codetether_session_store::Ticket, header: String) -> anyhow::Result<()> {
/// let session = codetether_session_store::restore_consumer::<S>(tx, ticket, header, ticket.to, 1)?;
/// # Ok(()) }
/// ```
pub fn restore_consumer<S: Restore>(
    tx: &Transaction<'_>,
    ticket: &Ticket,
    header: String,
    count: usize,
    revision: i64,
) -> Result<S> {
    let mut session = S::decode_header(&header)?;
    for body in crate::read::read_window(tx, &ticket.id, 0, ticket.from, ticket.to)? {
        let (message, page) = serde_json::from_str(&body)?;
        session.messages_mut().push(message);
        session.pages_mut().push(page);
    }
    {
        let mut state = session.state().0.lock().unwrap();
        state.read_only = true;
        state.id = ticket.id.clone();
        state.path = ticket.path.clone();
        state.revision = revision;
        state.message_start = ticket.from;
        state.message_end = count;
        state.header = header;
    }
    crate::clean(&session);
    Ok(session)
}
