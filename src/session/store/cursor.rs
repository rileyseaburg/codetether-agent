//! Durable independent consumers acknowledge only work they actually processed.
use crate::session::Session;
use anyhow::Result;
use std::path::PathBuf;
pub(crate) struct Delivery {
    pub session: Session,
    pub ticket: Ticket,
}
#[derive(Clone)]
pub(crate) struct Ticket {
    pub path: PathBuf,
    pub id: String,
    pub name: String,
    pub from: usize,
    pub to: usize,
    pub generation: i64,
}
pub(crate) async fn next(id: &str, name: &str, limit: usize) -> Result<Option<Delivery>> {
    let path = Session::session_path(id)?;
    let id = id.to_owned();
    let name = name.to_owned();
    tokio::task::spawn_blocking(move || super::cursor_read::next(&path, &id, &name, limit)).await?
}
pub(crate) async fn acknowledge(ticket: Ticket) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        let db = super::connection::open(&ticket.path)?;
        let rows = db.execute("UPDATE consumers SET seq=?3 WHERE session_id=?1 AND name=?2 AND revision=?4 AND seq<=?3",
            rusqlite::params![ticket.id, ticket.name, ticket.to as i64, ticket.generation])?;
        anyhow::Ok(rows == 1)
    }).await?
}
