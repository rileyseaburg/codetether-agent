//! Publish story results through nonblocking persistence tasks.
use super::*;
impl<H: Host> RalphLoop<H> {
    pub(crate) fn record_story_result(
        &self,
        story: &UserStory,
        passed: bool,
        error: Option<String>,
    ) {
        codetether_ralph_core::state_store::updates::story(
            self.store.as_ref(),
            &self.run_id,
            &self.state,
            story,
            passed,
            error,
        );
    }
}
