//! Concrete session operations required by the native Ralph relay.
use crate::{
    bus::AgentBus,
    provider::{Message, ProviderRegistry},
    session::{Session, SessionEvent, SessionResult},
};
use std::sync::Arc;
use tokio::sync::mpsc;
impl codetether_native_ralph::relay::Session for Session {
    async fn new() -> anyhow::Result<Self> {
        Self::new().await
    }
    fn configure(&mut self, model: String, name: String, bus: Arc<AgentBus>, message: Message) {
        self.metadata.model = Some(model);
        self.set_agent_name(name);
        self.bus = Some(bus);
        self.add_message(message);
    }
    async fn prompt_with_events(
        &mut self,
        prompt: &str,
        events: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
    ) -> anyhow::Result<SessionResult> {
        self.prompt_with_events(prompt, events, registry).await
    }
}
