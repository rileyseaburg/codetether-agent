//! get or create runtime for relay coordination.
use super::*;
impl RelayAutoChatTool {
    pub(super) async fn get_or_create_runtime(
        &self,
        relay_id: &str,
    ) -> Result<Arc<ProtocolRelayRuntime>> {
        if let Some(runtime) = {
            let store = RELAY_STORE.read();
            store.get(relay_id).map(|state| Arc::clone(&state.runtime))
        } {
            return Ok(runtime);
        }

        let bus = get_agent_bus().await?;
        let runtime = Arc::new(ProtocolRelayRuntime::with_relay_id(
            bus,
            relay_id.to_string(),
        ));
        let mut store = RELAY_STORE.write();
        let state = store
            .entry(relay_id.to_string())
            .or_insert_with(|| RelayRuntimeState::new(Arc::clone(&runtime)));
        Ok(Arc::clone(&state.runtime))
    }
}
