//! Preserve the application's session load, history and append behavior.
use crate::{provider::Message, session::Session};
#[async_trait::async_trait]
impl codetether_collaboration_history::HistorySource for Session {
    async fn load(parent_id: &str) -> anyhow::Result<Self> {
        Session::load(parent_id).await
    }
    fn history(&self) -> &[Message] {
        Session::history(self)
    }
    fn add_message(&mut self, message: Message) {
        Session::add_message(self, message)
    }
}
