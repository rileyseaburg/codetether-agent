//! Native consumer snapshot type and session-path adapter.
use crate::session::Session;
use anyhow::Result;
pub(crate) type Delivery = codetether_session_store::Delivery<Session>;
pub(crate) use codetether_session_store::acknowledge;

pub(crate) async fn next(id: &str, name: &str, limit: usize) -> Result<Option<Delivery>> {
    codetether_session_store::consumer::next_for::<Session>(id, name, limit).await
}
