//! Shared patch data types.

/// Parsed unified-diff hunk targeting one file.
///
/// # Examples
/// ```
/// use codetether_tool_core::patch::PatchHunk;
/// let hunk = PatchHunk { file: "note.txt".into(), start_line: 0, old_lines: vec![], new_lines: vec!["hello".into()] };
/// assert!(hunk.old_lines.is_empty());
/// ```
#[derive(Debug)]
pub struct PatchHunk {
    /// Target path as decoded from the file header.
    pub file: String,
    /// Old-range start parsed from the hunk header.
    pub start_line: usize,
    /// Removed and context lines used to locate the hunk.
    pub old_lines: Vec<String>,
    /// Added and context lines used in the replacement.
    pub new_lines: Vec<String>,
}
