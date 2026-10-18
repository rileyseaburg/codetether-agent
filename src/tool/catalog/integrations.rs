//! Native integrations tool construction.
use super::*;
use codetether_tool_construction::NativeTool;
pub(super) fn create(kind: NativeTool) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        NativeTool::McpBridge => Arc::new(mcp_bridge::McpBridgeTool::new()) as Arc<dyn Tool>,
        NativeTool::RustyRoad => Arc::new(rustyroad::RustyRoadTool) as Arc<dyn Tool>,
        #[cfg(feature = "tetherscript")]
        NativeTool::TetherScript => {
            Arc::new(tetherscript::TetherScriptPluginTool::new()) as Arc<dyn Tool>
        }
        _ => return None,
    })
}
