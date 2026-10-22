//! Native registry execution and shared tool-result recording.
use super::*;
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::tool_call::ToolCallHost;
pub(super) struct ApplicationHost<'a, S, H: Application> {
    pub session: &'a mut S,
    pub registry: &'a ToolRegistry<H>,
    pub workspace_dir: &'a Path,
    pub model: &'a str,
    pub cb: &'a Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}
#[async_trait]
impl<S: InputContext, H: Application> ToolCallHost for ApplicationHost<'_, S, H> {
    fn repeated_edit(&self, tool: &str, input: &serde_json::Value) -> Option<String> {
        self.session.repeated_edit(tool, input)
    }
    fn prepare_input(&mut self, input: &serde_json::Value) -> serde_json::Value {
        self.session
            .prepare_worker_input(input, self.workspace_dir, self.model)
    }
    async fn run_tool(
        &mut self,
        tool_name: &str,
        input: serde_json::Value,
    ) -> codetether_tool_core::ToolResult {
        crate::tools::run_tool(self.registry, tool_name, input, self.cb).await
    }
    async fn persist_context(&mut self) -> anyhow::Result<()> {
        transcript::Host::save(self.session).await
    }
    async fn refresh_context(&mut self) -> anyhow::Result<()> {
        self.session.refresh_context().await
    }
    fn record_result(&mut self, tool_id: String, result: codetether_tool_core::ToolResult) {
        self.session.record_outcome(&tool_id, &result);
        transcript::add_tool_result(self.session, tool_id, result);
    }
}
