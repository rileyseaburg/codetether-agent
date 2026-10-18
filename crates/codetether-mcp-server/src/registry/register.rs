//! Filter and register application tools for remote MCP clients.
use super::Host;
use codetether_mcp_core::server_catalog::ServerCatalog;
use std::sync::Arc;
/// Register the remotely usable tools from an application registry.
/// # Arguments
/// * `catalog` - Destination MCP catalog.
/// * `registry` - Existing tools and policy-aware handler factory.
/// # Returns
/// Registers all available tools except the established interactive/internal list.
/// # Examples
/// ```no_run
/// async fn setup<H: codetether_mcp_server::registry::Host>(catalog: &codetether_mcp_core::server_catalog::ServerCatalog, registry: &H) {
///     codetether_mcp_server::registry::register(catalog, registry).await;
/// }
/// ```
pub async fn register<H: Host>(catalog: &ServerCatalog, registry: &H) {
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

        catalog
            .register_tool(
                tool.id(),
                tool.description(),
                tool.parameters(),
                H::handler(tool_name, tool_clone),
            )
            .await;
    }
}
