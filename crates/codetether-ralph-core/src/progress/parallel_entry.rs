//! Build parallel progress evidence after the local response write.
use crate::{ProgressEntry, UserStory};
use std::path::Path;
/// Write successful output to its progress path and construct a progress record.
///
/// # Arguments
/// `story` and `final_result` describe the attempt; `story_working_dir` and
/// `progress_path` locate the response file.
/// # Returns
/// A completed or failed first-iteration record. File-write errors are ignored.
/// # Examples
/// ```rust,no_run
/// fn example(story: &codetether_ralph_core::UserStory) {
/// let entry = codetether_ralph_core::progress::parallel_entry(story, &Ok("done".into()),
///     std::path::Path::new("."), "progress.txt");
/// assert_eq!(entry.iteration, 1);
/// }
/// ```

pub fn parallel_entry(
    story: &UserStory,
    final_result: &anyhow::Result<String>,
    story_working_dir: &Path,
    progress_path: &str,
) -> ProgressEntry {
    let entry = match final_result {
        Ok(response) => {
            // Append progress to worktree-local progress file
            let progress_file = story_working_dir.join(progress_path);
            let _ = std::fs::write(&progress_file, response);

            super::completed_entry(
                story,
                1,
                response,
                crate::presentation::extract_learnings_static,
            )
        }
        Err(e) => {
            tracing::warn!(target: "codetether_agent::ralph::ralph_loop::parallel_result", "LLM call failed for story {}: {}", story.id, e);
            super::failed_entry(story, 1, e)
        }
    };

    entry
}
