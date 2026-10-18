//! Native identity, process approval and outgoing tool policy bindings.
use anyhow::Result;
use serde_json::Value;

/// Policy used by the public native client and registry aliases.
pub struct Native;
impl codetether_mcp_client::Application for Native {
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    async fn authorize(command: &str, args: &[&str], approval_id: Option<&str>) -> Result<()> {
        crate::mcp::subprocess_policy::guard(command, args, approval_id).await
    }
    async fn blocked(name: &str, arguments: &Value) -> Option<String> {
        crate::runtime_policy::evaluate_tool_invocation(name, arguments)
            .await
            .map(|result| result.output)
    }
}
