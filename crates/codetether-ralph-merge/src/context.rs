//! Borrowed workspace inputs for parallel-story merging.
use super::*;
pub(crate) struct MergeContext<'a> {
    pub(crate) provider: &'a Arc<dyn Provider>,
    pub(crate) model: &'a str,
    pub(crate) working_dir: &'a Path,
    pub(crate) story: &'a UserStory,
    pub(crate) wt: &'a WorktreeInfo,
    pub(crate) mgr: &'a Arc<WorktreeManager>,
}
