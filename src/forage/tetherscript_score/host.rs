//! Existing runtime policy and TetherScript tool bindings.
pub(super) struct Native;
impl codetether_forage_scoring::Host for Native {
    #[cfg(feature = "tetherscript")]
    async fn evaluate_tool_invocation(
        name: &str,
        args: &serde_json::Value,
    ) -> Option<crate::tool::ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(name, args).await
    }
    #[cfg(feature = "tetherscript")]
    async fn execute(args: serde_json::Value) -> anyhow::Result<crate::tool::ToolResult> {
        use crate::tool::{Tool, tetherscript::TetherScriptPluginTool};
        TetherScriptPluginTool::new().execute(args).await
    }
}
