use std::sync::Arc;

use crate::tool::ToolRegistry;

use super::TetherScriptPluginTool;

/// Register the dedicated TetherScript partner tool.
pub fn register(registry: &mut ToolRegistry) {
    registry.register(Arc::new(TetherScriptPluginTool::new()));
}
