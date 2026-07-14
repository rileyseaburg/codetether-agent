//! Registration of media + integration tools (podcast, youtube, avatar,
//! image, mcp bridge, okr, bus inspect) shared by every `ToolRegistry` builder.

use super::{
    Tool, ToolRegistry, alias, avatar, bus_inspect, image, image_generation, mcp_bridge, okr,
    podcast, youtube,
};
use std::sync::Arc;

/// Register the media + integration tool cluster onto `registry`.
pub(super) fn register(registry: &mut ToolRegistry) {
    registry.register(Arc::new(podcast::PodcastTool::new()));
    registry.register(Arc::new(youtube::YouTubeTool::new()));
    registry.register(Arc::new(avatar::AvatarTool::new()));
    registry.register(Arc::new(image::ImageTool::new()));
    let image_generation: Arc<dyn Tool> = Arc::new(image_generation::ImageGenerationTool::new());
    registry.register(Arc::clone(&image_generation));
    registry.register(Arc::new(alias::AliasTool::new(
        "imagegen",
        image_generation,
    )));
    registry.register(Arc::new(mcp_bridge::McpBridgeTool::new()));
    registry.register(Arc::new(okr::OkrTool::new()));
    registry.register(Arc::new(bus_inspect::BusInspectTool));
}
