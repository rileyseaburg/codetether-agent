//! Bind the shared recovery sequence to native durable sessions.
use crate::session::Session;
use codetether_session_recovery::RecoverySession;
use std::path::{Path, PathBuf};
impl codetether_session_recovery::http::HttpState for crate::server::AppState {
    fn server_url(&self) -> Option<&str> {
        self.config.a2a.server_url.as_deref()
    }
}
#[async_trait::async_trait]
impl RecoverySession for Session {
    async fn recorded_workspace(id: &str) -> anyhow::Result<PathBuf> {
        Session::recorded_workspace(id).await
    }
    async fn resume(id: &str) -> anyhow::Result<Self> {
        Session::resume(id).await
    }
    fn directory(&self) -> Option<&Path> {
        self.metadata.directory.as_deref()
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn set_agent_name(&mut self, agent: String) {
        Session::set_agent_name(self, agent);
    }
    fn set_model(&mut self, model: String) {
        self.metadata.model = Some(model);
    }
    async fn save(&self) -> anyhow::Result<()> {
        Session::save(self).await
    }
    async fn prompt(&mut self, prompt: &str) -> anyhow::Result<String> {
        Session::prompt(self, prompt)
            .await
            .map(|result| result.text)
    }
}
