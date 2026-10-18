//! Native session construction, persistence, bus and parent-policy operations.
use crate::session::Session;
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_session::ChildSession;
#[async_trait]
impl ChildSession for Session {
    async fn create() -> Result<Self> {
        Session::new().await
    }
    fn set_agent_name(&mut self, name: String) {
        Session::set_agent_name(self, name);
    }
    fn set_title(&mut self, title: String) {
        Session::set_title(self, title);
    }
    fn has_bus(&self) -> bool {
        self.bus.is_some()
    }
    fn attach_bus(&mut self) {
        self.bus = crate::bus::global();
    }
    async fn save_updated(&mut self) -> Result<()> {
        self.updated_at = chrono::Utc::now();
        self.save().await
    }
    async fn load_parent_policy(parent_id: &str) -> Result<bool> {
        let load = Session::load_tail(parent_id, 16).await?;
        Ok(crate::session::helper::runtime::prior_context_allowed_for_session(&load.session))
    }
}
