//! Adapt application tools to policy-checked MCP handlers.
use super::*;
impl McpServer {
    /// Bridge all tools from a ToolRegistry into MCP tool handlers.
    pub(super) async fn register_registry_tools(&self, registry: &Arc<ToolRegistry>) {
        // Skip tools that don't make sense over MCP:
        // - question: interactive TUI-only prompt
        // - invalid: internal error handler
        // - batch: needs weak registry reference, internal orchestration
        // - confirm_edit / confirm_multiedit: dead TUI confirmation tools
        // - plan_enter / plan_exit: session-state dependent TUI tools
        // - voice / podcast / youtube / avatar: media generation tools
        // - undo: git undo, dangerous without TUI context
        let skip_tools = [
            "question",
            "invalid",
            "batch",
            "confirm_edit",
            "confirm_multiedit",
            "plan_enter",
            "plan_exit",
            "voice",
            "podcast",
            "youtube",
            "avatar",
            "undo",
        ];

        for tool_id in registry.list() {
            if skip_tools.contains(&tool_id) {
                continue;
            }

            let tool = match registry.get(tool_id) {
                Some(t) => t,
                None => continue,
            };

            let tool_name = tool.id().to_string();
            let tool_clone = Arc::clone(&tool);

            self.register_tool(
                tool.id(),
                tool.description(),
                tool.parameters(),
                Arc::new(move |args: Value| {
                    super::super::tool_policy::call_registered(
                        tool_name.clone(),
                        Arc::clone(&tool_clone),
                        args,
                    )
                }),
            )
            .await;
        }
    }
}
