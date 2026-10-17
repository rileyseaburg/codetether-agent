//! Announce the selected workspace before running a story.
use super::parallel_task::ParallelTask;
use super::*;
impl ParallelTask {
    pub(super) async fn announce(&self, story_working_dir: &Path) {
        info!(
            "Working on story: {} - {} (in {:?})",
            self.story.id, self.story.title, story_working_dir
        );

        // Emit StoryStarted event
        if let Some(ref tx) = self.ralph_tx {
            let _ = tx
                .send(RalphEvent::StoryStarted {
                    story_id: self.story.id.clone(),
                })
                .await;
        }
    }
}
