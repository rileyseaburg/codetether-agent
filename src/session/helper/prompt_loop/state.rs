//! Aggregate state for one shared prompt-loop turn.

use std::sync::Arc;
use tokio::sync::mpsc;

use crate::cognition::tool_router::ToolCallRouter;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};

/// Complete mutable state for one user prompt turn.
pub(crate) struct Runner<'a> {
    /// Session receiving transcript and metadata updates.
    pub session: &'a mut Session,
    /// Optional lifecycle-event destination used by streaming callers.
    pub events: Option<mpsc::Sender<SessionEvent>>,
    /// Provider registry used for routing and failover.
    pub registry: Arc<ProviderRegistry>,
    /// Active provider and model settings.
    pub model: super::model::ModelState,
    /// Workspace paths tracked for post-edit validation.
    pub workspace: super::progress::WorkspaceState,
    /// Counters and output accumulated during the turn.
    pub progress: super::progress::LoopState,
    /// Optional response-to-tool-call formatter.
    pub router: Option<ToolCallRouter>,
    /// Watcher that injects completed sub-agent notices.
    pub subagents: super::super::prompt_events::subagent_watch::SubAgentWatch,
}

/// Decision returned after handling one completion response.
pub(crate) enum StepFlow {
    /// Continue with another provider step.
    Continue,
    /// Persist the session and return the accumulated answer.
    Finish,
}
