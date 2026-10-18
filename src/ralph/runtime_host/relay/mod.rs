//! Native registry and session bindings for shared relay orchestration.
//!
//! The relay crate owns provider selection and session turn ordering.
use crate::{
    bus::AgentBus,
    provider::{ContentPart, Message, ProviderRegistry, Role},
    session::{Session, SessionEvent},
};
use codetether_ralph_relay::Host;
use std::sync::Arc;
use tokio::sync::mpsc;
mod create;
mod prompt;
/// Native session creation and prompting for the shared runtime.
pub struct Native;
#[async_trait::async_trait]
impl Host for Native {
    type Registry = ProviderRegistry;
    type Session = Session;
    fn providers(registry: &ProviderRegistry) -> Vec<&str> {
        registry.list()
    }
    async fn new_session(
        model: String,
        name: &str,
        instructions: &str,
        bus: &Arc<AgentBus>,
    ) -> anyhow::Result<Session> {
        create::run(model, name, instructions, bus).await
    }
    async fn prompt(
        session: Session,
        baton: String,
        events: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
    ) -> (Session, anyhow::Result<crate::session::SessionResult>) {
        prompt::run(session, baton, events, registry).await
    }
}
