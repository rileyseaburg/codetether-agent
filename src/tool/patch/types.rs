//! Shared patch data types.

/// Parsed unified-diff hunk targeting one file.
#[derive(Debug)]
pub(super) struct PatchHunk {
    pub file: String,
    pub start_line: usize,
    pub old_lines: Vec<String>,
    pub new_lines: Vec<String>,
}
