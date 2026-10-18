//! Construct editing tools, accepting provider-bound editor factories.
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub(super) fn create(
    kind: ToolKind,
    edit: impl FnOnce() -> Arc<dyn Tool>,
    multi_edit: impl FnOnce() -> Arc<dyn Tool>,
) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::AdvancedEdit => {
            Arc::new(codetether_advanced_edit::AdvancedEditTool::new()) as Arc<dyn Tool>
        }
        ToolKind::Edit => edit(),
        ToolKind::MultiEdit => multi_edit(),
        ToolKind::ApplyPatch => Arc::new(codetether_patch::ApplyPatchTool::new()) as Arc<dyn Tool>,
        ToolKind::ConfirmEdit => {
            Arc::new(codetether_confirm_edit::ConfirmEditTool::new()) as Arc<dyn Tool>
        }
        ToolKind::ConfirmMultiEdit => {
            Arc::new(codetether_confirm_multiedit::ConfirmMultiEditTool::new()) as Arc<dyn Tool>
        }
        _ => return None,
    })
}
