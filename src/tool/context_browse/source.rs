//! Existing indexed transcript reads and metadata used by the shared browser.
use crate::{provider::Message, session::Session};
use std::path::Path;
#[async_trait::async_trait]
impl codetether_context_browse::BrowseSource for Session {
    fn id(&self) -> &str {
        &self.id
    }
    fn workspace(&self) -> Option<&Path> {
        self.metadata.directory.as_deref()
    }
    fn message_count(&self) -> usize {
        Session::message_count(self)
    }
    async fn read_messages(id: &str, start: usize, limit: usize) -> anyhow::Result<Vec<Message>> {
        Session::read_messages(id, start, limit).await
    }
}
