//! Native session storage binding for the shared HTTP handlers.
use crate::session::{Session, SessionResult, SessionSummary};
use codetether_session_http::SessionHost;
#[async_trait::async_trait]
impl SessionHost for Session {
    async fn new() -> anyhow::Result<Self> {
        Session::new().await
    }
    async fn load(id: &str) -> anyhow::Result<Self> {
        Session::load(id).await
    }
    async fn list() -> anyhow::Result<Vec<SessionSummary>> {
        crate::session::list_sessions().await
    }
    fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }
    fn set_agent_name(&mut self, agent: String) {
        Session::set_agent_name(self, agent);
    }
    async fn save(&self) -> anyhow::Result<()> {
        Session::save(self).await
    }
    async fn prompt(&mut self, message: &str) -> anyhow::Result<SessionResult> {
        Session::prompt(self, message).await
    }
}
