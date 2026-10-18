//! Native transcript and session-storage bindings for shared checkpoint persistence.
use crate::provider::Message;
use crate::session::Session;
impl codetether_provider_step::checkpoint::Host for Session {
    fn append(&mut self, message: Message) {
        self.add_message(message);
    }
    async fn save(&self) -> anyhow::Result<()> {
        Session::save(self).await
    }
}
