//! Native consumer snapshot type and session-path adapter.
use crate::session::Session;
use anyhow::Result;
pub(crate) type Delivery = codetether_session_store::Delivery<Session>;
pub(crate) use codetether_session_store::{Ticket, acknowledge};

pub(crate) async fn next(id: &str, name: &str, limit: usize) -> Result<Option<Delivery>> {
    let path = Session::session_path(id)?;
    let id = id.to_owned();
    let name = name.to_owned();
    tokio::task::spawn_blocking(move || super::cursor_read::next(&path, &id, &name, limit)).await?
}
