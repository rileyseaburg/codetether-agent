//! Native provider, sink and session bindings for the shared relay runtime.
use crate::{
    bus::AgentBus,
    provider::{ContentPart, Message, ProviderRegistry, Role},
    session::Session,
};
use std::sync::Arc;
pub(super) struct NativeHost;
#[async_trait::async_trait(?Send)]
impl codetether_cli_relay::Host for NativeHost {
    type Registry = ProviderRegistry;
    type Session = Session;
    fn spawn_bus_sink(bus: Arc<AgentBus>) {
        crate::bus::s3_sink::spawn_bus_s3_sink(bus);
    }
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
}
#[async_trait::async_trait(?Send)]
impl codetether_cli_relay::Session for Session {
    async fn new() -> anyhow::Result<Self> {
        Session::new().await
    }
    fn configure(&mut self, model: String, name: String, bus: Arc<AgentBus>, instructions: String) {
        self.metadata.model = Some(model);
        self.set_agent_name(name);
        self.bus = Some(bus);
        self.add_message(Message {
            role: Role::System,
            content: vec![ContentPart::Text { text: instructions }],
        });
    }
    async fn prompt(&mut self, prompt: &str) -> anyhow::Result<crate::session::SessionResult> {
        Session::prompt(self, prompt).await
    }
}
#[path = "relay_host_run.rs"]
mod run;
pub(super) use run::run_protocol_first_relay;
