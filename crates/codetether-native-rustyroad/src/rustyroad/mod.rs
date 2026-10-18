//! Bind RustyRoad to the application's workspace policy and handshake identity.
/// RustyRoad adapter using the application's existing authorization hook.
pub type RustyRoadTool = codetether_rustyroad_tool::RustyRoadTool<ApplicationHost>;
/// Preserve the existing unit-like tool construction used by the registries.
#[allow(non_upper_case_globals)]
pub const RustyRoadTool: RustyRoadTool = codetether_rustyroad_tool::RustyRoadTool::new();
/// Workspace authorization and application package identity.
pub struct ApplicationHost;
#[async_trait::async_trait]
impl codetether_rustyroad_tool::Host for ApplicationHost {
    const CLIENT_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    async fn evaluate_tool_invocation_for_workspace(
        tool: &str,
        input: &serde_json::Value,
        cwd: &std::path::Path,
    ) -> Option<crate::tool::ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation_for_workspace(tool, input, cwd).await
    }
}
#[cfg(test)]
mod tests;
