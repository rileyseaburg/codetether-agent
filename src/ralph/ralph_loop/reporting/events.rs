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
        codetether_ralph_core::events::initial_stories(prd)
    }
}
