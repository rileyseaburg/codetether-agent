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

/// Register the goals tool group in its original order.
///
/// # Arguments
/// * `host` — Native tool construction and registration bindings.
/// # Returns
/// Registers the group in its existing order.
/// # Examples
/// ```no_run
/// # fn setup(host: &mut impl codetether_tool_registry::catalog::Host) {
/// codetether_tool_registry::catalog::groups::goals(host);
/// # }
/// ```
pub fn goals(host: &mut impl Host) {
    for kind in [
        ToolKind::SessionTask,
        ToolKind::GetGoal,
        ToolKind::CreateGoal,
        ToolKind::UpdateGoal,
        ToolKind::EditGoal,
    ] {
        host.register(kind);
    }
}

/// Register the collaboration tool group in its original order.
///
/// # Arguments
/// * `host` — Native tool construction and registration bindings.
/// # Returns
/// Registers the group in its existing order.
/// # Examples
/// ```no_run
/// # fn setup(host: &mut impl codetether_tool_registry::catalog::Host) {
/// codetether_tool_registry::catalog::groups::collaboration(host);
/// # }
/// ```
pub fn collaboration(host: &mut impl Host) {
    for kind in [
        ToolKind::Agent,
        ToolKind::SpawnAgent,
        ToolKind::FollowupTask,
        ToolKind::ListAgents,
        ToolKind::WaitAgent,
        ToolKind::InterruptAgent,
        ToolKind::CloseAgent,
        ToolKind::ResumeAgent,
        ToolKind::SendInput,
        ToolKind::SendMessage,
    ] {
        host.register(kind);
    }
}

/// Register the scripting tool group in its original order.
///
/// # Arguments
/// * `host` — Native tool construction and registration bindings.
/// # Returns
/// Registers the group in its existing order.
/// # Examples
/// ```no_run
/// # fn setup(host: &mut impl codetether_tool_registry::catalog::Host) {
/// codetether_tool_registry::catalog::groups::scripting(host);
/// # }
/// ```
pub fn scripting(host: &mut impl Host) {
    for kind in [ToolKind::TetherScript] {
        host.register(kind);
    }
}
