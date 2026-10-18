//! Route Ralph events to their state reducers.
use crate::{RalphEvent, RalphViewState};
impl RalphViewState {
    /// Apply one run, story or completion event.
    ///
    /// # Examples
    /// ```
    /// use codetether_ralph_view::{RalphEvent,RalphViewState};
    /// let mut state = RalphViewState::new();
    /// state.handle_event(RalphEvent::Error("unavailable".into()));
    /// assert_eq!(state.error.as_deref(), Some("unavailable"));
    /// ```
    pub fn handle_event(&mut self, event: RalphEvent) {
        match event {
            event @ (RalphEvent::Started { .. } | RalphEvent::IterationStarted { .. }) => {
                self.apply_run_start(event)
            }
            event @ (RalphEvent::StoryStarted { .. }
            | RalphEvent::StoryToolCall { .. }
            | RalphEvent::StoryToolCallDetail { .. }) => self.apply_story_activity(event),
            event @ (RalphEvent::StoryQualityCheck { .. } | RalphEvent::StoryComplete { .. }) => {
                self.apply_story_checks(event)
            }
            event @ (RalphEvent::StoryOutput { .. }
            | RalphEvent::StoryError { .. }
            | RalphEvent::StoryMerge { .. }) => self.apply_story_results(event),
            event @ (RalphEvent::StageComplete { .. }
            | RalphEvent::Complete { .. }
            | RalphEvent::Error(..)) => self.apply_run_end(event),
            event @ (RalphEvent::StoryMessage { .. }) => self.apply_story_messages(event),
        }
    }
}
