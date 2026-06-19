//! Registration of media + integration tools (podcast, youtube, avatar,
//! image, mcp bridge, okr, bus inspect) shared by every `ToolRegistry` builder.

use super::{ToolRegistry, avatar, bus_inspect, image, mcp_bridge, okr, podcast, youtube};
use std::sync::Arc;

/// Register the media + integration tool cluster onto `registry`.
pub(super) fn register(registry: &mut ToolRegistry) {
    registry.register(Arc::new(podcast::PodcastTool::new()));
    registry.register(Arc::new(youtube::YouTubeTool::new()));
    registry.register(Arc::new(avatar::AvatarTool::new()));
    registry.register(Arc::new(image::ImageTool::new()));
    registry.register(Arc::new(mcp_bridge::McpBridgeTool::new()));
    registry.register(Arc::new(okr::OkrTool::new()));
    registry.register(Arc::new(bus_inspect::BusInspectTool));
}
