//! Ralph story checks event reduction.
use crate::{RalphEvent, RalphStoryStatus, RalphViewState};

impl RalphViewState {
    pub(crate) fn apply_story_checks(&mut self, event: RalphEvent) {
        match event {
            RalphEvent::StoryQualityCheck {
                story_id,
                check_name,
                passed,
            } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    story.status = RalphStoryStatus::QualityCheck;
                    story.current_tool = None;
                    story.quality_checks.push((check_name, passed));
                }
            }
            RalphEvent::StoryComplete { story_id, passed } => {
                if let Some(story) = self.stories.iter_mut().find(|s| s.id == story_id) {
                    story.status = if passed {
                        RalphStoryStatus::Passed
                    } else {
                        RalphStoryStatus::Failed
                    };
                    story.current_tool = None;
                }
            }
            _ => unreachable!("event routed to incorrect Ralph reducer"),
        }
    }
}
