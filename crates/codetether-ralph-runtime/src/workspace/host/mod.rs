//! Bind native commit, verification, state and reporting services for merges.
//!
//! The shared merge crate owns outcome routing and cleanup decisions.
use super::*;
#[async_trait::async_trait]
impl<H: Host> codetether_ralph_merge::Host for RalphLoop<H> {
    type Agent = H::Agent;
    fn state_mut(&mut self) -> &mut RalphState {
        &mut self.state
    }
    fn bus(&self) -> Option<Arc<AgentBus>> {
        self.bus.clone()
    }
    fn commit_in_dir(&self, directory: &PathBuf, story: &UserStory) -> anyhow::Result<()> {
        self.commit_in_dir(directory, story)
    }
    async fn verify(directory: &Path, story: &UserStory) -> anyhow::Result<()> {
        H::verify(directory, story).await
    }
    fn record_story_result(&self, story: &UserStory, passed: bool, error: Option<String>) {
        self.record_story_result(story, passed, error);
    }
    fn try_send_event(&self, event: RalphEvent) {
        self.try_send_event(event);
    }
}
