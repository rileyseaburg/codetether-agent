//! Borrowed workspace inputs for parallel-story merging.
use super::*;
pub(in crate::ralph::ralph_loop) struct MergeContext<'a> {
    pub(in crate::ralph::ralph_loop) provider: &'a Arc<dyn Provider>,
    pub(in crate::ralph::ralph_loop) model: &'a str,
    pub(in crate::ralph::ralph_loop) working_dir: &'a Path,
    pub(in crate::ralph::ralph_loop) story: &'a UserStory,
    pub(in crate::ralph::ralph_loop) wt: &'a crate::worktree::WorktreeInfo,
    pub(in crate::ralph::ralph_loop) mgr: &'a Arc<WorktreeManager>,
}
