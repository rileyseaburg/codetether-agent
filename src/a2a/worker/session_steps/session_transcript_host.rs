//! Native session operations for shared worker transcript updates.
use crate::{provider::Message, session::Session};
use async_trait::async_trait;
#[async_trait]
impl codetether_a2a_worker_core::session_protocol::transcript::Host for Session {
    fn append(&mut self, message: Message) {
        self.add_message(message);
    }
    async fn save(&mut self) -> anyhow::Result<()> {
        Session::save(self).await
    }
}
