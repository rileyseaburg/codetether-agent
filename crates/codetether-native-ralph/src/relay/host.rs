//! Shared relay host bound to the native provider registry.
use super::*;
#[async_trait::async_trait]
impl<S: Session> codetether_ralph_relay::Host for Native<S> {
    type Registry = ProviderRegistry;
    type Session = S;
    fn providers(registry: &ProviderRegistry) -> Vec<&str> {
        registry.list()
    }
    async fn new_session(
        model: String,
        name: &str,
        instructions: &str,
        bus: &Arc<AgentBus>,
    ) -> anyhow::Result<S> {
        create::run(model, name, instructions, bus).await
    }
    async fn prompt(
        session: S,
        baton: String,
        events: mpsc::Sender<SessionEvent<S>>,
        registry: Arc<ProviderRegistry>,
    ) -> (S, anyhow::Result<SessionResult>) {
        prompt::run(session, baton, events, registry).await
    }
}
