//! Send application events and build initial story presentation state.
use super::*;
impl RalphLoop {
    /// Non-blocking send of a Ralph event
    pub(in crate::ralph::ralph_loop) fn try_send_event(&self, event: RalphEvent) {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.try_send(event);
        }
    }

    /// Build initial RalphStoryInfo list from the PRD
    pub(in crate::ralph::ralph_loop) fn build_story_infos(prd: &Prd) -> Vec<RalphStoryInfo> {
        prd.user_stories
            .iter()
            .map(|s| RalphStoryInfo {
                id: s.id.clone(),
                title: s.title.clone(),
                status: if s.passes {
                    RalphStoryStatus::Passed
                } else {
                    RalphStoryStatus::Pending
                },
                priority: s.priority,
                depends_on: s.depends_on.clone(),
                quality_checks: Vec::new(),
                tool_call_history: Vec::new(),
                messages: Vec::new(),
                output: None,
                error: None,
                merge_summary: None,
                steps: 0,
                current_tool: None,
            })
            .collect()
    }
}
