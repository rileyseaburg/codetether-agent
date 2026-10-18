//! Native state access and the successful-story merge boundary.
use crate::*;
/// Services required to coordinate parallel stages.
/// # Implementors
/// Native runtimes lend their existing state and retain policy-aware merging.
#[async_trait::async_trait]
pub trait Host: Send + Sync {
    /// Native relay, model and verification bindings for spawned story tasks.
    type Task: TaskHost;
    /// Borrow live configuration and resource handles without cloning them.
    fn inputs(&self) -> Inputs<'_, Self::Task>;
    /// Mutate the live iteration and progress state.
    fn state_mut(&mut self) -> &mut RalphState;
    /// Merge an already verified story and deliver the native merge result.
    async fn merge_parallel_story(
        &mut self,
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
        story: &UserStory,
        worktree: Option<&WorktreeInfo>,
        manager: Option<&Arc<WorktreeManager>>,
    );
}
