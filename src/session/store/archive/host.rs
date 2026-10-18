//! Existing durable consumer and message view for the shared archive worker.
use crate::{provider::Message, session::Session};
use anyhow::Result;
use async_trait::async_trait;
use codetether_history_sink::archive::ArchiveSource;
use codetether_session_store::Delivery;
pub(super) struct ApplicationArchive;
#[async_trait]
impl ArchiveSource for ApplicationArchive {
    type Session = Session;
    async fn next(id: &str, name: &str, limit: usize) -> Result<Option<Delivery<Session>>> {
        super::super::cursor::next(id, name, limit).await
    }
    fn messages(session: &Session) -> &[Message] {
        &session.messages
    }
}
