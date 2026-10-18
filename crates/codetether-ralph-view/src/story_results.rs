//! Ralph story results event reduction.
use crate::{RalphEvent, RalphStoryStatus, RalphViewState};

impl RalphViewState {
    pub(crate) fn apply_story_results(&mut self, event: RalphEvent) {
        match event {
            RalphEvent::StoryOutput { story_id, output } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    codetether_view_retention::agent_update::output(
                        &mut story.output,
                        &output,
                        "Ralph output",
                    );
                }
            }
            RalphEvent::StoryError { story_id, error } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    story.error = Some(error.clone());
                    story.status = RalphStoryStatus::Failed;
                }
            }
            RalphEvent::StoryMerge {
                story_id,
                success: _,
                summary,
            } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    story.merge_summary = Some(summary);
                }
            }
            _ => unreachable!("event routed to incorrect Ralph reducer"),
        }
    }
}
