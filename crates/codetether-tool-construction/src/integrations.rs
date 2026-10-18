//! Construct integrations tools for the ordered catalog.
use crate::{Host, NativeTool};
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub(super) fn create(kind: ToolKind, host: &impl Host) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::Podcast => Arc::new(codetether_podcast_tool::PodcastTool::new()) as Arc<dyn Tool>,
        ToolKind::YouTube => Arc::new(codetether_youtube_tool::YouTubeTool::new()) as Arc<dyn Tool>,
        ToolKind::Avatar => Arc::new(codetether_avatar_tool::AvatarTool::new()) as Arc<dyn Tool>,
        ToolKind::Image => Arc::new(codetether_image_tool::ImageTool::new()) as Arc<dyn Tool>,
        ToolKind::ImageGeneration => {
            Arc::new(codetether_image_generation::ImageGenerationTool::new()) as Arc<dyn Tool>
        }
        ToolKind::McpBridge => host.create(NativeTool::McpBridge),
        ToolKind::RustyRoad => host.create(NativeTool::RustyRoad),
        ToolKind::Okr => Arc::new(codetether_okr_tool::OkrTool::new()) as Arc<dyn Tool>,
        ToolKind::BusInspect => Arc::new(codetether_bus_tool::BusInspectTool) as Arc<dyn Tool>,
        ToolKind::ComputerUse => {
            Arc::new(codetether_computer_tool::ComputerUseTool::new()) as Arc<dyn Tool>
        }
        #[cfg(feature = "tetherscript")]
        ToolKind::TetherScript => host.create(NativeTool::TetherScript),
        _ => return None,
    })
}
