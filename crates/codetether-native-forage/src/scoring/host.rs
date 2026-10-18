//! Existing runtime policy and TetherScript tool bindings.
pub(super) struct Native;
impl codetether_forage_scoring::Host for Native {
    #[cfg(feature = "tetherscript")]
    async fn evaluate_tool_invocation(
        name: &str,
        args: &serde_json::Value,
    ) -> Option<codetether_tool_core::ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(name, args).await
    }
    #[cfg(feature = "tetherscript")]
    async fn execute(args: serde_json::Value) -> anyhow::Result<codetether_tool_core::ToolResult> {
        use codetether_native_tetherscript::TetherScriptPluginTool;
        use codetether_tool_core::Tool;
        TetherScriptPluginTool::new().execute(args).await
    }
}
