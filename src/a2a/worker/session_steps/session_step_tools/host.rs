//! Session-specific effects for shared worker tool-call coordination.
use super::*;
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::tool_call::ToolCallHost;
pub(super) struct ApplicationHost<'a> {
    pub session: &'a mut Session,
    pub registry: &'a crate::tool::ToolRegistry,
    pub workspace_dir: &'a Path,
    pub model: &'a str,
    pub cb: &'a Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}
#[async_trait]
impl ToolCallHost for ApplicationHost<'_> {
    fn prepare_input(&mut self, input: &serde_json::Value) -> serde_json::Value {
        super::super::super::enrich_tool_input_for_session_model(
            input,
            self.workspace_dir,
            self.session,
            Some(self.model),
        )
    }
    async fn run_tool(
        &mut self,
        tool_name: &str,
        input: serde_json::Value,
    ) -> crate::tool::ToolResult {
        codetether_native_worker::tools::run_tool(self.registry, tool_name, input, self.cb).await
    }
    fn record_result(&mut self, tool_id: String, result: crate::tool::ToolResult) {
        session_output::add_tool_result(self.session, tool_id, result);
    }
}
