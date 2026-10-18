//! Native session lifecycle and provider bindings for the shared A2A server.
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent, SessionResult};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
/// Native application services used by the public A2A server alias.
pub struct Native;
impl codetether_a2a_server::Application for Native {
    type Session = Session;
    type Registry = Arc<ProviderRegistry>;
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    async fn resolve(context_id: Option<&str>) -> Result<Session> {
        crate::a2a::session_resolve::resolve_session(context_id).await
    }
    async fn configure(session: &mut Session) {
        crate::a2a::session_config::configure(session).await;
    }
    async fn registry() -> Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
    async fn execute(
        session: &mut Session,
        prompt: &str,
        sender: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
    ) -> Result<String> {
        session
            .prompt_with_events(prompt, sender, registry)
            .await
            .map(|result| result.text)
    }
    async fn prompt(session: &mut Session, prompt: &str) -> Result<SessionResult> {
        crate::a2a::prompt_runtime::run(session, prompt).await
    }
    async fn persist(session: &Session) {
        crate::a2a::session_config::persist(session).await;
    }
}
