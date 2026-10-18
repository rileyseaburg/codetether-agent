//! Application session-aware enrichment for native worker tool calls.
use crate::session::Session;
impl codetether_native_worker::tool_call::InputContext for Session {
    fn prepare_worker_input(
        &self,
        input: &serde_json::Value,
        workspace: &std::path::Path,
        model: &str,
    ) -> serde_json::Value {
        super::super::super::enrich_tool_input_for_session_model(
            input,
            workspace,
            self,
            Some(model),
        )
    }
}
