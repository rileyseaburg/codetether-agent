//! Native integrations tool construction.
use super::*;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::Podcast => Arc::new(podcast::PodcastTool::new()) as Arc<dyn Tool>,
        ToolKind::YouTube => Arc::new(youtube::YouTubeTool::new()) as Arc<dyn Tool>,
        ToolKind::Avatar => Arc::new(avatar::AvatarTool::new()) as Arc<dyn Tool>,
        ToolKind::Image => Arc::new(image::ImageTool::new()) as Arc<dyn Tool>,
        ToolKind::ImageGeneration => {
            Arc::new(image_generation::ImageGenerationTool::new()) as Arc<dyn Tool>
        }
        ToolKind::McpBridge => Arc::new(mcp_bridge::McpBridgeTool::new()) as Arc<dyn Tool>,
        ToolKind::RustyRoad => Arc::new(rustyroad::RustyRoadTool) as Arc<dyn Tool>,
        ToolKind::Okr => Arc::new(okr::OkrTool::new()) as Arc<dyn Tool>,
        ToolKind::BusInspect => Arc::new(bus_inspect::BusInspectTool) as Arc<dyn Tool>,
        ToolKind::ComputerUse => Arc::new(computer_use::ComputerUseTool::new()) as Arc<dyn Tool>,
        #[cfg(feature = "tetherscript")]
        ToolKind::TetherScript => {
            Arc::new(tetherscript::TetherScriptPluginTool::new()) as Arc<dyn Tool>
        }
        _ => return None,
    })
}
