//! Concrete message-window access for native proactive workers.
use crate::provider::Message;
use crate::session::Session;
impl codetether_native_proactive::Session for Session {
    fn into_messages(self) -> Vec<Message> {
        self.messages.into_vec()
    }
    async fn load_messages(id: &str, limit: usize) -> anyhow::Result<Vec<Message>> {
        let loaded = Self::load_tail(id, limit).await?;
        Ok(loaded.session.messages.into_vec())
    }
}
