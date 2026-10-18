//! Ralph story messages event reduction.
use crate::{RalphEvent, RalphViewState};

impl RalphViewState {
    pub(crate) fn apply_story_messages(&mut self, event: RalphEvent) {
        match event {
            RalphEvent::StoryMessage { story_id, entry } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    codetether_view_retention::agent_update::push(&mut story.messages, entry);
                }
            }
            _ => unreachable!("event routed to incorrect Ralph reducer"),
        }
    }
}
