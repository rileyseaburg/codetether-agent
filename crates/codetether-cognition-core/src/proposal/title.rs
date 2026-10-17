//! Short proposal titles derived from a thought.
use crate::text as text_util;

/// Derive a short proposal title from a thought's first non-empty line.
///
/// # Arguments
/// * `thought` - Completed thought and its provenance.
/// * `thought_count` - Thought sequence used by the empty-title fallback.
///
/// # Returns
/// A compact first-line title capped for storage.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::proposal::title::proposal_title_from_thought;
/// assert_eq!(proposal_title_from_thought("  Review change  ", 1), "Review change");
/// ```
pub fn proposal_title_from_thought(thought: &str, thought_count: u64) -> String {
    let first_line = thought
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("proposal");
    let compact = first_line
        .replace(['\t', '\r', '\n'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let trimmed = text_util::trim_for_storage(&compact, 72);
    if trimmed.is_empty() {
        format!("proposal-{thought_count}")
    } else {
        trimmed
    }
}
