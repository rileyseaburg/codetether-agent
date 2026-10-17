//! Sequential Ralph iteration handling.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn begin_sequential_iteration(&mut self) -> Option<UserStory> {
        self.state.current_iteration += 1;

        // Persist iteration update
        codetether_ralph_core::state_store::updates::iteration(
            self.store.as_ref(),
            &self.run_id,
            &self.state,
        );

        info!(target: "codetether_agent::ralph::ralph_loop::seq_iteration",
            "=== Ralph iteration {} of {} ===",
            self.state.current_iteration, self.state.max_iterations
        );

        // Emit iteration event
        self.try_send_event(RalphEvent::IterationStarted {
            iteration: self.state.current_iteration,
            max_iterations: self.state.max_iterations,
        });

        // Check if all stories are complete
        if self.state.prd.is_complete() {
            info!(target: "codetether_agent::ralph::ralph_loop::seq_iteration", "All stories complete!");
            self.state.status = RalphStatus::Completed;
            return None;
        }

        // Get next story to work on
        let story = match self.state.prd.next_story() {
            Some(s) => s.clone(),
            None => {
                warn!(target: "codetether_agent::ralph::ralph_loop::seq_iteration", "No available stories (dependencies not met)");
                return None;
            }
        };

        info!(target: "codetether_agent::ralph::ralph_loop::seq_iteration", "Working on story: {} - {}", story.id, story.title);

        // Emit StoryStarted event
        self.try_send_event(RalphEvent::StoryStarted {
            story_id: story.id.clone(),
        });

        Some(story)
    }
}
