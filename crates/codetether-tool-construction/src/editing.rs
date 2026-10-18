//! Edit and multi-edit construction with the application's Morph backend.
use crate::NativeTool;
use codetether_morph_backend::Editor;
use codetether_tool_core::Tool;
use std::sync::Arc;

/// Construct an edit-family tool using the selected Morph editor.
/// # Arguments
/// * `kind` - Catalog identity requested by the ordered registry builder.
/// # Returns
/// The matching edit tool, or `None` for a kind owned by another factory.
/// # Examples
/// ```no_run
/// # fn example<E: codetether_morph_backend::Editor + Default + 'static>() {
/// let tool = codetether_tool_construction::editing::create::<E>(
///     codetether_tool_construction::NativeTool::Edit);
/// assert!(tool.is_some());
/// # }
/// ```
pub fn create<E: Editor + Default + 'static>(kind: NativeTool) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        NativeTool::Edit => Arc::new(codetether_edit::EditTool::<E>::new()) as Arc<dyn Tool>,
        NativeTool::MultiEdit => {
            Arc::new(codetether_multiedit::MultiEditTool::<E>::new()) as Arc<dyn Tool>
        }
        _ => return None,
    })
}
