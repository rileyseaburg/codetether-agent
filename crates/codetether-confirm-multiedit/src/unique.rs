//! Require one exact occurrence of the requested original text.
use super::truncate::truncate_with_ellipsis;
use super::{EditOperation, ToolResult};
pub(super) fn validate(content: &str, edit: &EditOperation) -> Option<ToolResult> {
    let matches: Vec<_> = content.match_indices(&edit.old_string).collect();

    if matches.is_empty() {
        return Some(ToolResult::error(format!(
            "String not found in {}: {}",
            edit.file,
            truncate_with_ellipsis(&edit.old_string, 50)
        )));
    }

    if matches.len() > 1 {
        return Some(ToolResult::error(format!(
            "String found {} times in {} (must be unique). Use more context to disambiguate.",
            matches.len(),
            edit.file
        )));
    }

    None
}
