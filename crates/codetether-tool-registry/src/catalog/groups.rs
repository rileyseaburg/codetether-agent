//! Ordered search, media and compatibility registration groups.
use super::{Host, ToolKind};
/// Register the existing search-tool group in its original order.
///
/// # Arguments
/// * `host` — Native tool construction and registration bindings.
/// # Returns
/// Registers this group in the original order.
/// # Examples
/// ```no_run
/// # fn setup(host: &mut impl codetether_tool_registry::catalog::Host) {
/// codetether_tool_registry::catalog::groups::search(host);
/// # }
/// ```
pub fn search(host: &mut impl Host) {
    for kind in [
        ToolKind::Grep,
        ToolKind::Ripgrep,
        ToolKind::CodeSearch,
        ToolKind::TokenCount,
    ] {
        host.register(kind);
    }
}
/// Register media integrations, preserving the shared image-generation instance.
///
/// # Arguments
/// * `host` — Native tool construction and registration bindings.
/// # Returns
/// Registers this group in the original order.
/// # Examples
/// ```no_run
/// # fn setup(host: &mut impl codetether_tool_registry::catalog::Host) {
/// codetether_tool_registry::catalog::groups::media(host);
/// # }
/// ```
pub fn media(host: &mut impl Host) {
    for kind in [
        ToolKind::Podcast,
        ToolKind::YouTube,
        ToolKind::Avatar,
        ToolKind::Image,
    ] {
        host.register(kind);
    }
    host.register_with_alias("imagegen", ToolKind::ImageGeneration);
    for kind in [
        ToolKind::McpBridge,
        ToolKind::RustyRoad,
        ToolKind::Okr,
        ToolKind::BusInspect,
    ] {
        host.register(kind);
    }
}
/// Register fresh compatibility instances and then the computer-use capability.
///
/// # Arguments
/// * `host` — Native tool construction and registration bindings.
/// # Returns
/// Registers this group in the original order.
/// # Examples
/// ```no_run
/// # fn setup(host: &mut impl codetether_tool_registry::catalog::Host) {
/// codetether_tool_registry::catalog::groups::compatibility(host);
/// # }
/// ```
pub fn compatibility(host: &mut impl Host) {
    for (id, kind) in [
        ("patch", ToolKind::ApplyPatch),
        ("file_info", ToolKind::FileInfo),
        ("head_tail", ToolKind::HeadTail),
        ("todo_read", ToolKind::TodoRead),
        ("todo_write", ToolKind::TodoWrite),
        ("mcp_bridge", ToolKind::McpBridge),
        ("k8s_tool", ToolKind::K8s),
    ] {
        host.register_alias(id, kind);
    }
    host.register(ToolKind::ComputerUse);
}
