//! Parallel task result retained until stage collection and merge.
use crate::*;
/// Story, model success, progress, worktree, manager, quality status and failure.
/// # Examples
/// ```rust
/// # fn example(story: codetether_ralph_core::UserStory, progress: codetether_ralph_core::ProgressEntry) {
/// let result: codetether_ralph_task::StoryResult = (story, true, progress, None, None, true, None);
/// assert!(result.1 && result.5);
/// # }
/// ```
pub type StoryResult = (
    UserStory,
    bool,
    ProgressEntry,
    Option<WorktreeInfo>,
    Option<Arc<WorktreeManager>>,
    bool,
    Option<String>,
);
