//! Owned inputs retained by one spawned parallel story task.
use super::*;
/// Inputs captured for one parallel story, retained through result assembly.
/// # Examples
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use codetether_ralph_task::{Host, Task};
/// # use codetether_provider_core::Provider;
/// # use codetether_ralph_core::{UserStory, QualityChecks};
/// # fn example<H: Host>(provider: Arc<dyn Provider>, story: UserStory) {
/// let task = Task::<H> {
///     sem: Arc::new(tokio::sync::Semaphore::new(1)), provider,
///     model: "selected-model".into(), prd_info: ("project".into(), "feature".into()),
///     working_dir: ".".into(), worktree_mgr: None, prebuilt_worktree: None,
///     progress_path: "progress.txt".into(), ralph_tx: None,
///     stage_learnings: vec![], bus: None, relay_enabled: false,
///     relay_max_agents: 2, relay_max_rounds: 1, registry: None,
///     max_steps_per_story: 30, max_quality_retries: 2,
///     max_rate_limit_retries: 2, rate_limit_base_delay_ms: 1000,
///     quality_checks_enabled: false, quality_checks: QualityChecks::default(),
///     delegation: None, story,
/// };
/// assert_eq!(task.max_steps_per_story, 30);
/// # }
/// ```
pub struct ParallelTask<H: Host> {
    /// Shared stage concurrency limit.
    pub sem: Arc<tokio::sync::Semaphore>,
    /// Selected model provider.
    pub provider: Arc<dyn Provider>,
    /// Selected model name.
    pub model: String,
    /// Project and feature names used by the story prompt.
    pub prd_info: (String, String),
    /// Original workspace or selected worktree root.
    pub working_dir: PathBuf,
    /// Manager retained until the stage merges or reports this result.
    pub worktree_mgr: Option<Arc<WorktreeManager>>,
    /// Optional pre-created isolated checkout for this story.
    pub prebuilt_worktree: Option<WorktreeInfo>,
    /// Progress file path resolved within the story workspace.
    pub progress_path: String,
    /// Optional story lifecycle event sender.
    pub ralph_tx: Option<mpsc::Sender<RalphEvent>>,
    /// Accumulated learnings captured before this stage started.
    pub stage_learnings: Vec<String>,
    /// Optional shared agent event bus.
    pub bus: Option<Arc<AgentBus>>,
    /// Whether to select relay execution when a registry is available.
    pub relay_enabled: bool,
    /// Requested relay team size.
    pub relay_max_agents: usize,
    /// Requested relay round limit.
    pub relay_max_rounds: usize,
    /// Optional provider registry retained for relay execution.
    pub registry: Option<Arc<H::Registry>>,
    /// Step budget for the initial single-agent execution.
    pub max_steps_per_story: usize,
    /// Maximum quality repair retries.
    pub max_quality_retries: usize,
    /// Maximum model rate-limit retries.
    pub max_rate_limit_retries: usize,
    /// Initial rate-limit retry delay in milliseconds.
    pub rate_limit_base_delay_ms: u64,
    /// Whether to run configured quality commands before verification.
    pub quality_checks_enabled: bool,
    /// Commands used by the quality repair loop.
    pub quality_checks: QualityChecks,
    /// Optional provider-selection calibration state.
    pub delegation: Option<DelegationState>,
    /// Story whose requirements and identity drive this task.
    pub story: UserStory,
}
