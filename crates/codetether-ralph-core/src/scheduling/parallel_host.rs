//! Host interface for dependency-stage scheduling.
use super::*;
/// Supply stage planning and complete stage execution with its owned resources.
///
/// # Implementors
/// Runtime adapters retain task handles and semaphore ownership until stage
/// collection and finalization finish.
#[async_trait::async_trait]
pub trait ParallelHost: Send + Sync {
    /// The manager retained across all stages without additional cloning.
    type Manager: Send + Sync;
    /// Return owned dependency-ordered story groups.
    fn stages(&self) -> Vec<Vec<UserStory>>;
    /// Construct optional worktree isolation after planning stages.
    fn manager(&self) -> Option<Self::Manager>;
    /// Check limits and announce a stage; return false to stop.
    fn begin(&mut self, index: usize, total: usize, stories: &[UserStory]) -> bool;
    /// Create worktrees, run and collect tasks, then finalize a stage.
    async fn execute_stage(
        &mut self,
        stories: Vec<UserStory>,
        manager: &Option<Self::Manager>,
    ) -> Result<()>;
}
