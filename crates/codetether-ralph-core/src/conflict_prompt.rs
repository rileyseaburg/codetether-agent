//! Format the original conflict-resolution task and file diffs.
use crate::UserStory;
use std::path::Path;
/// Build the conflict resolver prompt from story context and conflicting diffs.
///
/// # Arguments
/// * `story` - Story being merged.
/// * `conflicts` - Conflicting file names.
/// * `conflict_diffs` - File names paired with diff contents.
/// * `working_dir` - Workspace displayed in the prompt.
/// # Returns
/// Resolver instructions preserving original formatting and ordering.
/// # Examples
/// ```
/// # fn example(story: &codetether_ralph_core::UserStory) {
/// let prompt = codetether_ralph_core::conflict_prompt::build(story, &[], &[], std::path::Path::new("."));
/// assert!(prompt.contains("CONFLICT RESOLUTION TASK"));
/// # }
/// ```
pub fn build(
    story: &UserStory,
    conflicts: &[String],
    conflict_diffs: &[(String, String)],
    working_dir: &Path,
) -> String {
    // Build prompt with conflict context
    let conflict_info = conflict_diffs
        .iter()
        .map(|(file, diff)| format!("### File: {}\n```diff\n{}\n```", file, diff))
        .collect::<Vec<_>>()
        .join("\n\n");

    let prompt = format!(
        include_str!("conflict_prompt.txt"),
        story.id,
        story.title,
        story.description,
        conflicts
            .iter()
            .map(|f| format!("- {}", f))
            .collect::<Vec<_>>()
            .join("\n"),
        conflict_info,
        story.id,
        working_dir.display()
    );

    prompt
}
