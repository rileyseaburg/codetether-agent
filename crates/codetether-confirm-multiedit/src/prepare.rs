//! Validate all files and prepare ordered preview records.
use super::load::FileContent;
use super::{EditOperation, EditPreview, ToolResult};
use anyhow::Result;
pub(super) async fn load(edits: &[EditOperation]) -> Result<Result<Vec<FileContent>, ToolResult>> {
    let mut file_contents = Vec::new();
    for edit in edits {
        match super::load::load(edit).await? {
            Ok(file) => file_contents.push(file),
            Err(error) => return Ok(Err(error)),
        }
    }
    Ok(Ok(file_contents))
}
pub(super) fn previews(file_contents: &[FileContent]) -> (Vec<EditPreview>, usize, usize) {
    let mut previews = Vec::new();
    let mut total_added = 0;
    let mut total_removed = 0;
    for (path, content, old_string, new_string) in file_contents {
        let new_content = content.replacen(old_string, new_string, 1);
        let (diff_output, added, removed) = super::diff::preview(content, &new_content);
        previews.push(EditPreview {
            file: path.display().to_string(),
            diff: diff_output.trim().to_string(),
            added,
            removed,
        });

        total_added += added;
        total_removed += removed;
    }
    (previews, total_added, total_removed)
}
