//! Session-independent ownership of one prompt runner's state.
use crate::{LoopState, WorkspaceState};
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Complete mutable state for one prompt turn with application-supplied services.
///
/// The generic session, registry, model, router, and watcher keep native service
/// types outside this crate. Applications may wrap the state to implement their
/// own execution hooks.
/// # Examples
/// ```
/// use codetether_prompt_loop::{LoopState, RunnerState, WorkspaceState};
/// let mut session = ();
/// let state = RunnerState {
///     session: &mut session, events: None, registry: std::sync::Arc::new(()),
///     lease_owner: "session:turn".into(), model: (),
///     workspace: WorkspaceState {
///         cwd: ".".into(), baseline_dirty: Default::default(), touched: Default::default(),
///     },
///     progress: LoopState::new(None), router: None::<()>, subagents: (),
/// };
/// assert!(state.workspace.touched.is_empty());
/// ```
pub struct RunnerState<'a, S, R, M, T, W> {
    /// Session receiving transcript and metadata updates.
    pub session: &'a mut S,
    /// Optional lifecycle-event destination used by streaming callers.
    pub events: Option<mpsc::Sender<SessionEvent<S>>>,
    /// Provider registry used for routing and failover.
    pub registry: Arc<R>,
    /// Unique owner identity for mux leases held by this prompt run.
    pub lease_owner: String,
    /// Active provider and model settings.
    pub model: M,
    /// Workspace paths tracked for post-edit validation.
    pub workspace: WorkspaceState,
    /// Counters and output accumulated during the turn.
    pub progress: LoopState,
    /// Optional response-to-tool-call formatter.
    pub router: Option<T>,
    /// Watcher that injects completed sub-agent notices.
    pub subagents: W,
}
