//! Native files tool constructors.
use super::*;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::Read => Arc::new(file::ReadTool::new()) as Arc<dyn Tool>,
        ToolKind::Write => Arc::new(file::WriteTool::new()) as Arc<dyn Tool>,
        ToolKind::List => Arc::new(file::ListTool::new()) as Arc<dyn Tool>,
        ToolKind::Glob => Arc::new(file::GlobTool::new()) as Arc<dyn Tool>,
        ToolKind::Tree => Arc::new(file_extras::TreeTool::new()) as Arc<dyn Tool>,
        ToolKind::FileInfo => Arc::new(file_extras::FileInfoTool::new()) as Arc<dyn Tool>,
        ToolKind::HeadTail => Arc::new(file_extras::HeadTailTool::new()) as Arc<dyn Tool>,
        ToolKind::Diff => Arc::new(file_extras::DiffTool::new()) as Arc<dyn Tool>,
        ToolKind::Git => Arc::new(git::GitTool::new()) as Arc<dyn Tool>,
        ToolKind::AdvancedEdit => Arc::new(advanced_edit::AdvancedEditTool::new()) as Arc<dyn Tool>,
        ToolKind::Edit => Arc::new(edit::EditTool::new()) as Arc<dyn Tool>,
        ToolKind::MultiEdit => Arc::new(multiedit::MultiEditTool::new()) as Arc<dyn Tool>,
        ToolKind::ApplyPatch => Arc::new(patch::ApplyPatchTool::new()) as Arc<dyn Tool>,
        ToolKind::ConfirmEdit => Arc::new(confirm_edit::ConfirmEditTool::new()) as Arc<dyn Tool>,
        ToolKind::ConfirmMultiEdit => {
            Arc::new(confirm_multiedit::ConfirmMultiEditTool::new()) as Arc<dyn Tool>
        }
        _ => return None,
    })
}
