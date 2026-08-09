//! Unified patch generation from user-edited proposal buffers.

use similar::TextDiff;

use super::ApprovalEditFile;

pub(super) fn build(files: &[ApprovalEditFile]) -> String {
    files
        .iter()
        .filter(|file| file.original != file.revised)
        .map(file_patch)
        .collect::<Vec<_>>()
        .join("")
}

fn file_patch(file: &ApprovalEditFile) -> String {
    let old = format!("a/{}", file.relative);
    let new = format!("b/{}", file.relative);
    TextDiff::from_lines(&file.original, &file.revised)
        .unified_diff()
        .context_radius(3)
        .header(&old, &new)
        .to_string()
}

#[cfg(test)]
#[path = "patch_tests.rs"]
mod tests;
