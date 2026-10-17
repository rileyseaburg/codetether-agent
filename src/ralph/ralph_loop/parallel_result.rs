//! Build task progress entries and preserve the worktree-local response write.
use super::*;
impl RalphLoop {
    pub(super) fn parallel_progress_entry(
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

                ProgressEntry {
                    story_id: story.id.clone(),
                    iteration: 1,
                    status: "completed".to_string(),
                    learnings: Self::extract_learnings_static(response),
                    files_changed: Vec::new(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }
            }
            Err(e) => {
                warn!("LLM call failed for story {}: {}", story.id, e);
                ProgressEntry {
                    story_id: story.id.clone(),
                    iteration: 1,
                    status: format!("failed: {}", e),
                    learnings: Vec::new(),
                    files_changed: Vec::new(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }
            }
        };

        entry
    }
}
