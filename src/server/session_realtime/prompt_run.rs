//! Native session and provider bindings for shared realtime execution.
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent, SessionMetadata, SessionResult};
use std::sync::Arc;
use tokio::sync::mpsc;

pub(in crate::server) struct AppHost;
impl codetether_session_realtime::Host for AppHost {
    type Session = Session;
    type Registry = ProviderRegistry;
    async fn load(id: &str) -> anyhow::Result<Session> {
        Session::load(id).await
    }
    fn metadata(session: &mut Session) -> &mut SessionMetadata {
        &mut session.metadata
    }
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
    async fn prompt(
        session: &mut Session,
        message: &str,
        events: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
    ) -> anyhow::Result<SessionResult> {
        session.prompt_with_events(message, events, registry).await
    }
    async fn save(session: &Session) -> anyhow::Result<()> {
        session.save().await
    }
}
