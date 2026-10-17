//! Existing task result shape and borrowed merge context.
use super::*;
pub(super) type ParallelStoryResult = (
    UserStory,
    bool,
    ProgressEntry,
    Option<crate::worktree::WorktreeInfo>,
    Option<Arc<WorktreeManager>>,
    bool,
    Option<String>,
);
pub(super) struct MergeContext<'a> {
    pub(super) provider: &'a Arc<dyn Provider>,
    pub(super) model: &'a str,
    pub(super) working_dir: &'a Path,
    pub(super) story: &'a UserStory,
    pub(super) wt: &'a crate::worktree::WorktreeInfo,
    pub(super) mgr: &'a Arc<WorktreeManager>,
}
