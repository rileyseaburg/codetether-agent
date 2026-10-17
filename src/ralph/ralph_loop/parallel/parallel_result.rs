//! Build task progress entries and preserve the worktree-local response write.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn parallel_progress_entry(
        story: &UserStory,
        final_result: &anyhow::Result<String>,
        story_working_dir: &Path,
        progress_path: &str,
    ) -> ProgressEntry {
        codetether_ralph_core::progress::parallel_entry(
            story,
            final_result,
            story_working_dir,
            progress_path,
        )
    }
}
