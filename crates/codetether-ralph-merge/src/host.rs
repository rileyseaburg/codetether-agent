//! Native services used by the merge state machine.
use crate::*;
/// Commit, verify and report merged stories using the native runtime policy.
/// # Implementors
/// Native Ralph binds its existing commit helper, verification runner and event/store services.
#[async_trait::async_trait]
pub trait Host: Send + Sync {
    /// Existing prompt, tools and model loop for the conflict resolver.
    type Agent: codetether_ralph_agent::Host;
    /// Mutate the live PRD only after a completion branch verifies successfully.
    fn state_mut(&mut self) -> &mut RalphState;
    /// Clone the optional event bus at the existing resolver invocation boundary.
    fn bus(&self) -> Option<Arc<AgentBus>>;
    /// Commit changes in an isolated story workspace using native policy.
    /// # Errors
    /// Returns native commit-policy errors.
    fn commit_in_dir(&self, directory: &PathBuf, story: &UserStory) -> anyhow::Result<()>;
    /// Verify the main workspace against the story's completion criteria.
    /// # Errors
    /// Returns verification failures or command-runner errors.
    async fn verify(directory: &Path, story: &UserStory) -> anyhow::Result<()>;
    /// Record a story result through the existing nonblocking persistence service.
    fn record_story_result(&self, story: &UserStory, passed: bool, error: Option<String>);
    /// Try to deliver the original story event without blocking.
    fn try_send_event(&self, event: RalphEvent);
}
