//! Native session binding for shared edited-conversation branching.
use crate::provider::Message;
use crate::session::{Session, SessionMetadata};
#[async_trait::async_trait]
impl codetether_session_fork::ForkSession for Session {
    async fn new() -> anyhow::Result<Self> {
        Session::new().await
    }
    async fn load(id: &str) -> anyhow::Result<Self> {
        Session::load(id).await
    }
    async fn save(&self) -> anyhow::Result<()> {
        Session::save(self).await
    }
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn message_offset(&self) -> usize {
        Session::message_offset(self)
    }
    fn page_kinds(&self) -> &[crate::session::pages::PageKind] {
        &self.pages
    }
    fn replace_pages(&mut self, pages: Vec<crate::session::pages::PageKind>) {
        self.pages = pages.into();
    }
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    fn metadata(&self) -> &SessionMetadata {
        &self.metadata
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn agent(&self) -> &str {
        &self.agent
    }
    fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }
    fn set_metadata(&mut self, metadata: SessionMetadata) {
        self.metadata = metadata;
    }
    fn set_agent_name(&mut self, agent: &str) {
        Session::set_agent_name(self, agent);
    }
    fn add_message(&mut self, message: Message) {
        Session::add_message(self, message);
    }
}
