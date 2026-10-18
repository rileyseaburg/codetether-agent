//! Bind proactive workers to native session and cursor storage.
use crate::provider::Message;
use crate::session::Session;
use anyhow::Result;
use async_trait::async_trait;
use codetether_prepared_context::Prepared;
use codetether_session_store::{Delivery, Ticket};
use serde_json::Value;
pub(super) struct NativeHost;
#[async_trait]
impl codetether_proactive_worker::Host for NativeHost {
    type Session = Session;
    fn message_count(session: &Session) -> usize {
        session.messages.len()
    }
    fn into_messages(session: Session) -> Vec<Message> {
        session.messages.into_vec()
    }
    async fn next(
        &self,
        id: &str,
        consumer: &str,
        limit: usize,
    ) -> Result<Option<Delivery<Session>>> {
        crate::session::store::cursor::next(id, consumer, limit).await
    }
    async fn commit(&self, ticket: Ticket, view: Value) -> Result<()> {
        crate::session::store::projection::commit(ticket, view).await
    }
    async fn load_tail(&self, id: &str) -> Result<Vec<Message>> {
        let loaded = Session::load_tail(id, crate::session::store::WINDOW).await?;
        Ok(loaded.session.messages.into_vec())
    }
    async fn read_prepared(&self, id: &str) -> Option<Prepared> {
        super::super::store::read_id(id).await
    }
    async fn write_prepared(&self, id: &str, value: &Prepared) -> Result<()> {
        super::super::store::write(id, value).await
    }
    async fn remove_prepared(&self, id: &str) {
        super::super::store::remove(id).await;
    }
}
