//! Owned state produced by prompt initialization.
use crate::{LoopState, WorkspaceState};

/// Prepared turn state, independent of the borrowed session and registry.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_prompt_loop::setup::Host>(host: &mut H) -> anyhow::Result<()> {
/// let parts = codetether_prompt_loop::setup::initialize(host).await?;
/// assert!(parts.workspace.touched.is_empty());
/// # Ok(()) }
/// ```
pub struct Parts<M, R, W> {
    /// Unique session-prefixed lease identity.
    pub lease_owner: String,
    /// Resolved model and provider state.
    pub model: M,
    /// Effective workspace and its initial dirty paths.
    pub workspace: WorkspaceState,
    /// Fresh per-turn counters and guards.
    pub progress: LoopState,
    /// Optional tool-call router.
    pub router: Option<R>,
    /// Fresh sub-agent completion watcher.
    pub subagents: W,
}
