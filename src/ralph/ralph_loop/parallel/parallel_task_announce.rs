//! Announce the selected workspace before running a story.
use super::parallel_task::ParallelTask;
use super::*;
impl ParallelTask {
    pub(in crate::ralph::ralph_loop) async fn announce(&self, story_working_dir: &Path) {
        info!(target: "codetether_agent::ralph::ralph_loop::parallel_task_announce",
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
