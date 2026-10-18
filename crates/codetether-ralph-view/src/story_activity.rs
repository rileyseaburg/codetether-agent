//! Ralph story activity event reduction.
use crate::{RalphEvent, RalphStoryStatus, RalphViewState};

impl RalphViewState {
    pub(crate) fn apply_story_activity(&mut self, event: RalphEvent) {
        match event {
            RalphEvent::StoryStarted { story_id } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    story.status = RalphStoryStatus::Running;
                    story.steps = 0;
                    story.current_tool = None;
                    story.quality_checks.clear();
                    story.tool_call_history.clear();
                    story.messages.clear();
                    story.output = None;
                    story.error = None;
                }
            }
            RalphEvent::StoryToolCall {
                story_id,
                tool_name,
            } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    story.current_tool = Some(tool_name);
                    story.steps += 1;
                }
            }
            RalphEvent::StoryToolCallDetail { story_id, detail } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    story.current_tool = Some(detail.tool_name.clone());
                    story.steps += 1;
                    codetether_view_retention::agent_update::push(
                        &mut story.tool_call_history,
                        detail,
                    );
                }
            }
            _ => unreachable!("event routed to incorrect Ralph reducer"),
        }
    }
}
