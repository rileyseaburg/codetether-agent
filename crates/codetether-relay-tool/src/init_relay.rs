//! init relay for relay coordination.
use super::*;
impl RelayAutoChatTool {
    /// Initialize a new relay with a task
    pub(super) async fn init_relay(
        &self,
        relay_id: Option<String>,
        task: Option<String>,
        _context: Option<Value>,
        okr_id: Option<String>,
    ) -> Result<ToolResult> {
        let task = task.unwrap_or_else(|| "Unspecified task".to_string());
        let relay_id =
            relay_id.unwrap_or_else(|| format!("relay-{}", &Uuid::new_v4().to_string()[..8]));

        let bus = get_agent_bus().await?;
        let runtime = Arc::new(ProtocolRelayRuntime::with_relay_id(bus, relay_id.clone()));

        // Store the runtime
        {
            let mut store = RELAY_STORE.write();
            store.insert(relay_id.clone(), RelayRuntimeState::new(runtime));
        }

        let response = json!({
            "status": "initialized",
            "relay_id": relay_id,
            "task": task,
            "okr_id": okr_id,
            "message": "Relay initialized. Use 'delegate' to assign tasks to agents, or 'list_agents' to see available agents."
        });

        let mut result = ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        )
        .with_metadata("relay_id", json!(relay_id));
        if let Some(okr_id) = response.get("okr_id").and_then(|v| v.as_str()) {
            result = result.with_metadata("okr_id", json!(okr_id));
        }

        Ok(result)
    }
}
