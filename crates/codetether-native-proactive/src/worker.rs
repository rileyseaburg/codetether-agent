//! Proactive worker snapshots, transactional cursors and projection access.
use crate::{Host, Session, store};
use codetether_provider_core::Message;
use codetether_session_store::{Delivery, Ticket};
use serde_json::Value;
#[async_trait::async_trait]
impl<S: Session> codetether_proactive_worker::Host for Host<S> {
    type Session = S;
    fn message_count(session: &S) -> usize {
        codetether_proactive_context::Source::messages(session).len()
    }
    fn into_messages(session: S) -> Vec<Message> {
        Session::into_messages(session)
    }
    async fn next(
        &self,
        id: &str,
        consumer: &str,
        limit: usize,
    ) -> anyhow::Result<Option<Delivery<S>>> {
        crate::cursor::next::<S>(id, consumer, limit).await
    }
    async fn commit(&self, ticket: Ticket, view: Value) -> anyhow::Result<()> {
        codetether_session_store::projection::commit(ticket, view)
            .await
            .map(|_| ())
    }
    async fn load_tail(&self, id: &str) -> anyhow::Result<Vec<Message>> {
        S::load_messages(id, codetether_session_store::window::WINDOW).await
    }
    async fn read_prepared(&self, id: &str) -> Option<store::Prepared> {
        store::read_id(id).await
    }
    async fn write_prepared(&self, id: &str, value: &store::Prepared) -> anyhow::Result<()> {
        store::write(id, value).await
    }
    async fn remove_prepared(&self, id: &str) {
        store::remove(id).await;
    }
}
#[async_trait::async_trait]
impl<S: Session> codetether_proactive_context::Host for Host<S> {
    async fn projection(&self, id: &str, name: &str) -> anyhow::Result<Option<Value>> {
        codetether_session_store::projection::read_session(id, name).await
    }
}
